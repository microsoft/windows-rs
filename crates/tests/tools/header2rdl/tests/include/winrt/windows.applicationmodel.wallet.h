
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
#ifndef __windows2Eapplicationmodel2Ewallet_h__
#define __windows2Eapplicationmodel2Ewallet_h__
#ifndef __windows2Eapplicationmodel2Ewallet_p_h__
#define __windows2Eapplicationmodel2Ewallet_p_h__


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
#include "Windows.Devices.Geolocation.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletBarcode;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode ABI::Windows::ApplicationModel::Wallet::IWalletBarcode

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletBarcodeFactory;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory ABI::Windows::ApplicationModel::Wallet::IWalletBarcodeFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItemCustomProperty;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomProperty

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItemCustomPropertyFactory;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomPropertyFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItemFactory;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory ABI::Windows::ApplicationModel::Wallet::IWalletItemFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItemStore;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore ABI::Windows::ApplicationModel::Wallet::IWalletItemStore

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItemStore2;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2 ABI::Windows::ApplicationModel::Wallet::IWalletItemStore2

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletManagerStatics;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics ABI::Windows::ApplicationModel::Wallet::IWalletManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletRelevantLocation;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation ABI::Windows::ApplicationModel::Wallet::IWalletRelevantLocation

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletTransaction;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction ABI::Windows::ApplicationModel::Wallet::IWalletTransaction

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletVerb;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb ABI::Windows::ApplicationModel::Wallet::IWalletVerb

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletVerbFactory;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory ABI::Windows::ApplicationModel::Wallet::IWalletVerbFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletItem;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletItemStore;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9664f3ba-0049-5cbf-845f-8f0bcad2b14c"))
IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*, ABI::Windows::ApplicationModel::Wallet::IWalletItemStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Wallet.WalletItemStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5334975e-205a-5b6c-96fd-896fb93949bd"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*, ABI::Windows::ApplicationModel::Wallet::IWalletItemStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Wallet.WalletItemStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_USE */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("65178d50-e6a2-5d16-b244-65e9725e5a0c"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60847289-ea0b-5df6-89df-f2c62cba9693"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStreamReference>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStreamReference*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletItemCustomProperty;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1aa9cd86-9376-5ebb-b45c-8dad7e66f9f7"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*, ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomProperty*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletItemCustomProperty>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c66d71b4-4336-5693-836e-4915303c183b"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletItemCustomProperty>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f45154c-0c0d-5df9-a557-259f20c927ae"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletItemCustomProperty>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletRelevantLocation;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("031812fc-a4f7-5127-9ec4-d92621cb3f90"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*, ABI::Windows::ApplicationModel::Wallet::IWalletRelevantLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletRelevantLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b5d9a611-5f2c-542c-ae58-276753bbf8c7"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletRelevantLocation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e55e510c-4028-5df0-b78b-27bd06980b0b"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletRelevantLocation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletTransaction;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f50232e9-7ce3-559b-baad-6726ffa4e79b"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletTransaction*, ABI::Windows::ApplicationModel::Wallet::IWalletTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("75f5591e-31a9-50e9-b9d0-373b1eb0d6b6"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletTransaction>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4fd2ca58-b7a2-5923-9380-49c11c69c39a"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletTransaction>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletVerb;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1fe3a179-ffa8-5f52-9823-9c3dff73c56a"))
IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletVerb*, ABI::Windows::ApplicationModel::Wallet::IWalletVerb*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletVerb>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93b2ca58-f717-56e6-a945-8513c48f915c"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletVerb>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6d2ef172-8ae3-572e-ae44-8ee3fd49d19f"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Wallet.WalletVerb>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("338f0d64-dd4e-5614-b16f-0bd08d19fe02"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*, ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomProperty*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Wallet.WalletItemCustomProperty>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b5622af3-7c67-5158-b39a-4323488e6f87"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*, ABI::Windows::ApplicationModel::Wallet::IWalletRelevantLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Wallet.WalletRelevantLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05b768c4-0c95-5305-9b44-ef006b53300f"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletTransaction*, ABI::Windows::ApplicationModel::Wallet::IWalletTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Wallet.WalletTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93ca6ab8-a827-5e28-b21e-01a597457c3e"))
IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> : IMapView_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletVerb*, ABI::Windows::ApplicationModel::Wallet::IWalletVerb*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Wallet.WalletVerb>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#define DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5cc135b0-29f3-5113-a097-25e41a32e473"))
IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*, ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomProperty*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.ApplicationModel.Wallet.WalletItemCustomProperty>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletItemCustomProperty*> __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t;
#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#define DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9378c55a-852d-5ddf-b01b-9cc3d47ec52d"))
IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*, ABI::Windows::ApplicationModel::Wallet::IWalletRelevantLocation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.ApplicationModel.Wallet.WalletRelevantLocation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletRelevantLocation*> __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t;
#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#define DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3ef47bcf-8328-5245-8c91-f0ab4c399027"))
IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletTransaction*, ABI::Windows::ApplicationModel::Wallet::IWalletTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.ApplicationModel.Wallet.WalletTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletTransaction*> __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t;
#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#define DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bda656b0-1139-5fd3-9dfd-d24dbb910509"))
IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> : IMap_impl<HSTRING, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletVerb*, ABI::Windows::ApplicationModel::Wallet::IWalletVerb*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, Windows.ApplicationModel.Wallet.WalletVerb>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, ABI::Windows::ApplicationModel::Wallet::WalletVerb*> __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t;
#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_USE */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07fc9084-9ea1-5505-89cc-4d754719f582"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*, ABI::Windows::ApplicationModel::Wallet::IWalletItemStore*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Wallet.WalletItemStore, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Wallet::WalletItemStore*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Geolocation {
                typedef struct BasicGeoposition BasicGeoposition;
            } /* Geolocation */
        } /* Devices */
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
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                typedef enum WalletBarcodeSymbology : int WalletBarcodeSymbology;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                typedef enum WalletDetailViewPosition : int WalletDetailViewPosition;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                typedef enum WalletItemKind : int WalletItemKind;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                typedef enum WalletSummaryViewPosition : int WalletSummaryViewPosition;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletBarcode;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletActionKind
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                enum
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                DEPRECATED("WalletActionKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
                WalletActionKind : int
                {
                    WalletActionKind_OpenItem = 0,
                    WalletActionKind_Transaction = 1,
                    WalletActionKind_MoreTransactions = 2,
                    WalletActionKind_Message = 3,
                    WalletActionKind_Verb = 4,
                };
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletBarcodeSymbology
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("WalletBarcodeSymbology is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                WalletBarcodeSymbology : int
                {
                    WalletBarcodeSymbology_Invalid = 0,
                    WalletBarcodeSymbology_Upca = 1,
                    WalletBarcodeSymbology_Upce = 2,
                    WalletBarcodeSymbology_Ean13 = 3,
                    WalletBarcodeSymbology_Ean8 = 4,
                    WalletBarcodeSymbology_Itf = 5,
                    WalletBarcodeSymbology_Code39 = 6,
                    WalletBarcodeSymbology_Code128 = 7,
                    WalletBarcodeSymbology_Qr = 8,
                    WalletBarcodeSymbology_Pdf417 = 9,
                    WalletBarcodeSymbology_Aztec = 10,
                    WalletBarcodeSymbology_Custom = 100000,
                };
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletDetailViewPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("WalletDetailViewPosition is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                WalletDetailViewPosition : int
                {
                    WalletDetailViewPosition_Hidden = 0,
                    WalletDetailViewPosition_HeaderField1 = 1,
                    WalletDetailViewPosition_HeaderField2 = 2,
                    WalletDetailViewPosition_PrimaryField1 = 3,
                    WalletDetailViewPosition_PrimaryField2 = 4,
                    WalletDetailViewPosition_SecondaryField1 = 5,
                    WalletDetailViewPosition_SecondaryField2 = 6,
                    WalletDetailViewPosition_SecondaryField3 = 7,
                    WalletDetailViewPosition_SecondaryField4 = 8,
                    WalletDetailViewPosition_SecondaryField5 = 9,
                    WalletDetailViewPosition_CenterField1 = 10,
                    WalletDetailViewPosition_FooterField1 = 11,
                    WalletDetailViewPosition_FooterField2 = 12,
                    WalletDetailViewPosition_FooterField3 = 13,
                    WalletDetailViewPosition_FooterField4 = 14,
                };
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("WalletItemKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                WalletItemKind : int
                {
                    WalletItemKind_Invalid = 0,
                    WalletItemKind_Deal = 1,
                    WalletItemKind_General = 2,
                    WalletItemKind_PaymentInstrument = 3,
                    WalletItemKind_Ticket = 4,
                    WalletItemKind_BoardingPass = 5,
                    WalletItemKind_MembershipCard = 6,
                };
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletSummaryViewPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("WalletSummaryViewPosition is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                WalletSummaryViewPosition : int
                {
                    WalletSummaryViewPosition_Hidden = 0,
                    WalletSummaryViewPosition_Field1 = 1,
                    WalletSummaryViewPosition_Field2 = 2,
                };
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletBarcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletBarcode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletBarcode[] = L"Windows.ApplicationModel.Wallet.IWalletBarcode";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("4f857b29-de80-4ea4-a1cd-81cd084dac27")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletBarcode : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Symbology(
                        ABI::Windows::ApplicationModel::Wallet::WalletBarcodeSymbology* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE GetImageAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletBarcode = __uuidof(IWalletBarcode);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletBarcodeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletBarcode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletBarcodeFactory[] = L"Windows.ApplicationModel.Wallet.IWalletBarcodeFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("30117161-ed9c-469e-bbfd-306c95ea7108")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletBarcodeFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE CreateWalletBarcode(
                        ABI::Windows::ApplicationModel::Wallet::WalletBarcodeSymbology symbology,
                        HSTRING value,
                        ABI::Windows::ApplicationModel::Wallet::IWalletBarcode** barcode
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE CreateCustomWalletBarcode(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* streamToBarcodeImage,
                        ABI::Windows::ApplicationModel::Wallet::IWalletBarcode** barcode
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletBarcodeFactory = __uuidof(IWalletBarcodeFactory);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItem[] = L"Windows.ApplicationModel.Wallet.IWalletItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("20b54be8-118d-4ec4-996c-b963e7bd3e74")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItem : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IsAcknowledged(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IsAcknowledged(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IssuerDisplayName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IssuerDisplayName(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdated(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_LastUpdated(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::Wallet::WalletItemKind* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Barcode(
                        ABI::Windows::ApplicationModel::Wallet::IWalletBarcode** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Barcode(
                        ABI::Windows::ApplicationModel::Wallet::IWalletBarcode* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Logo159x159(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Logo159x159(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Logo336x336(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Logo336x336(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Logo99x99(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Logo99x99(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayMessage(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayMessage(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisplayMessageLaunchable(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IsDisplayMessageLaunchable(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_LogoText(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_LogoText(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_HeaderColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_BodyColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_BodyColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderFontColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_HeaderFontColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_BodyFontColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_BodyFontColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderBackgroundImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_HeaderBackgroundImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_BodyBackgroundImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_BodyBackgroundImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_LogoImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_LogoImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_PromotionalImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_PromotionalImage(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_RelevantDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_RelevantDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_RelevantDateDisplayMessage(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_RelevantDateDisplayMessage(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_TransactionHistory(
                        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_RelevantLocations(
                        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IsMoreTransactionHistoryLaunchable(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IsMoreTransactionHistoryLaunchable(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayProperties(
                        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Verbs(
                        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItem = __uuidof(IWalletItem);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemCustomProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemCustomProperty[] = L"Windows.ApplicationModel.Wallet.IWalletItemCustomProperty";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("b94b40f3-fa00-40fd-98dc-9de46697f1e7")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItemCustomProperty : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_AutoDetectLinks(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_AutoDetectLinks(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DetailViewPosition(
                        ABI::Windows::ApplicationModel::Wallet::WalletDetailViewPosition* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DetailViewPosition(
                        ABI::Windows::ApplicationModel::Wallet::WalletDetailViewPosition value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_SummaryViewPosition(
                        ABI::Windows::ApplicationModel::Wallet::WalletSummaryViewPosition* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_SummaryViewPosition(
                        ABI::Windows::ApplicationModel::Wallet::WalletSummaryViewPosition value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItemCustomProperty = __uuidof(IWalletItemCustomProperty);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemCustomPropertyFactory[] = L"Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("d0046a44-61a1-41aa-b259-a5610ab5d575")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItemCustomPropertyFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE CreateWalletItemCustomProperty(
                        HSTRING name,
                        HSTRING value,
                        ABI::Windows::ApplicationModel::Wallet::IWalletItemCustomProperty** walletItemCustomProperty
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItemCustomPropertyFactory = __uuidof(IWalletItemCustomPropertyFactory);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemFactory[] = L"Windows.ApplicationModel.Wallet.IWalletItemFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("53e27470-4f0b-4a3e-99e5-0bbb1eab38d4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItemFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItemFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE CreateWalletItem(
                        ABI::Windows::ApplicationModel::Wallet::WalletItemKind kind,
                        HSTRING displayName,
                        ABI::Windows::ApplicationModel::Wallet::IWalletItem** walletItem
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItemFactory = __uuidof(IWalletItemFactory);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemStore[] = L"Windows.ApplicationModel.Wallet.IWalletItemStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("7160484b-6d49-48f8-91a9-40a1d0f13ef4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItemStore : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE AddAsync(
                        HSTRING id,
                        ABI::Windows::ApplicationModel::Wallet::IWalletItem* item,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE ClearAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE GetWalletItemAsync(
                        HSTRING id,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE GetItemsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE GetItemsWithKindAsync(
                        ABI::Windows::ApplicationModel::Wallet::WalletItemKind kind,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE ImportItemAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* stream,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        HSTRING id,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE ShowAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE ShowItemAsync(
                        HSTRING id,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE UpdateAsync(
                        ABI::Windows::ApplicationModel::Wallet::IWalletItem* item,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItemStore = __uuidof(IWalletItemStore);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemStore2[] = L"Windows.ApplicationModel.Wallet.IWalletItemStore2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("65e682f0-7009-4a15-bd54-4fff379bffe2")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletItemStore2 : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE add_ItemsChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE remove_ItemsChanged(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletItemStore2 = __uuidof(IWalletItemStore2);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletManagerStatics[] = L"Windows.ApplicationModel.Wallet.IWalletManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("5111d6b8-c9a4-4c64-b4dd-e1e548001c0d")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletManagerStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletManagerStatics = __uuidof(IWalletManagerStatics);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletRelevantLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletRelevantLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletRelevantLocation[] = L"Windows.ApplicationModel.Wallet.IWalletRelevantLocation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("9fd8782a-e3f9-4de1-bab3-bb192e46b3f3")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletRelevantLocation : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Position(
                        ABI::Windows::Devices::Geolocation::BasicGeoposition value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayMessage(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayMessage(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletRelevantLocation = __uuidof(IWalletRelevantLocation);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletTransaction[] = L"Windows.ApplicationModel.Wallet.IWalletTransaction";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("40e1e940-2606-4519-81cb-bff1c60d1f79")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletTransaction : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Description(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayAmount(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayAmount(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IgnoreTimeOfDay(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IgnoreTimeOfDay(
                        boolean value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayLocation(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayLocation(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_TransactionDate(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_TransactionDate(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_IsLaunchable(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_IsLaunchable(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletTransaction = __uuidof(IWalletTransaction);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletVerb
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletVerb
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletVerb[] = L"Windows.ApplicationModel.Wallet.IWalletVerb";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("17b826d6-e3c1-4c74-8a94-217aadbc4884")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletVerb : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletVerb = __uuidof(IWalletVerb);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletVerbFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletVerb
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletVerbFactory[] = L"Windows.ApplicationModel.Wallet.IWalletVerbFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                MIDL_INTERFACE("76012771-be58-4d5e-83ed-58b1669c7ad9")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                DEPRECATED("IWalletVerbFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                IWalletVerbFactory : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletVerbFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    virtual HRESULT STDMETHODCALLTYPE CreateWalletVerb(
                        HSTRING name,
                        ABI::Windows::ApplicationModel::Wallet::IWalletVerb** WalletVerb
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWalletVerbFactory = __uuidof(IWalletVerbFactory);
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletBarcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletBarcodeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletBarcode ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletBarcode_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletBarcode_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletBarcode[] = L"Windows.ApplicationModel.Wallet.WalletBarcode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItem_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItem[] = L"Windows.ApplicationModel.Wallet.WalletItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItemCustomProperty ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemCustomProperty_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemCustomProperty_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItemCustomProperty[] = L"Windows.ApplicationModel.Wallet.WalletItemCustomProperty";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItemStore ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemStore_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItemStore[] = L"Windows.ApplicationModel.Wallet.WalletItemStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Wallet.IWalletManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletManager[] = L"Windows.ApplicationModel.Wallet.WalletManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletRelevantLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletRelevantLocation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletRelevantLocation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletRelevantLocation_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletRelevantLocation[] = L"Windows.ApplicationModel.Wallet.WalletRelevantLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletTransaction ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletTransaction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletTransaction_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletTransaction[] = L"Windows.ApplicationModel.Wallet.WalletTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletVerb
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletVerbFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletVerb ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletVerb_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletVerb_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletVerb[] = L"Windows.ApplicationModel.Wallet.WalletVerb";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2 __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemStore_INTERFACE_DEFINED__
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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty;

typedef struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl;

interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation;

typedef struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl;

interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction;

typedef struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl;

interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb;

typedef struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb* This);

    END_INTERFACE
} __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl;

interface __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb
{
    CONST_VTBL struct __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletBarcodeSymbology __x_ABI_CWindows_CApplicationModel_CWallet_CWalletBarcodeSymbology;

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletDetailViewPosition __x_ABI_CWindows_CApplicationModel_CWallet_CWalletDetailViewPosition;

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind __x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletSummaryViewPosition __x_ABI_CWindows_CApplicationModel_CWallet_CWalletSummaryViewPosition;

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletActionKind
 *
 * Introduced to Windows.ApplicationModel.Wallet.WalletContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
DEPRECATED("WalletActionKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CApplicationModel_CWallet_CWalletActionKind
{
    WalletActionKind_OpenItem = 0,
    WalletActionKind_Transaction = 1,
    WalletActionKind_MoreTransactions = 2,
    WalletActionKind_Message = 3,
    WalletActionKind_Verb = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletBarcodeSymbology
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletBarcodeSymbology is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CWalletBarcodeSymbology
{
    WalletBarcodeSymbology_Invalid = 0,
    WalletBarcodeSymbology_Upca = 1,
    WalletBarcodeSymbology_Upce = 2,
    WalletBarcodeSymbology_Ean13 = 3,
    WalletBarcodeSymbology_Ean8 = 4,
    WalletBarcodeSymbology_Itf = 5,
    WalletBarcodeSymbology_Code39 = 6,
    WalletBarcodeSymbology_Code128 = 7,
    WalletBarcodeSymbology_Qr = 8,
    WalletBarcodeSymbology_Pdf417 = 9,
    WalletBarcodeSymbology_Aztec = 10,
    WalletBarcodeSymbology_Custom = 100000,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletDetailViewPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletDetailViewPosition is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CWalletDetailViewPosition
{
    WalletDetailViewPosition_Hidden = 0,
    WalletDetailViewPosition_HeaderField1 = 1,
    WalletDetailViewPosition_HeaderField2 = 2,
    WalletDetailViewPosition_PrimaryField1 = 3,
    WalletDetailViewPosition_PrimaryField2 = 4,
    WalletDetailViewPosition_SecondaryField1 = 5,
    WalletDetailViewPosition_SecondaryField2 = 6,
    WalletDetailViewPosition_SecondaryField3 = 7,
    WalletDetailViewPosition_SecondaryField4 = 8,
    WalletDetailViewPosition_SecondaryField5 = 9,
    WalletDetailViewPosition_CenterField1 = 10,
    WalletDetailViewPosition_FooterField1 = 11,
    WalletDetailViewPosition_FooterField2 = 12,
    WalletDetailViewPosition_FooterField3 = 13,
    WalletDetailViewPosition_FooterField4 = 14,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemKind is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind
{
    WalletItemKind_Invalid = 0,
    WalletItemKind_Deal = 1,
    WalletItemKind_General = 2,
    WalletItemKind_PaymentInstrument = 3,
    WalletItemKind_Ticket = 4,
    WalletItemKind_BoardingPass = 5,
    WalletItemKind_MembershipCard = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Wallet.WalletSummaryViewPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletSummaryViewPosition is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CWalletSummaryViewPosition
{
    WalletSummaryViewPosition_Hidden = 0,
    WalletSummaryViewPosition_Field1 = 1,
    WalletSummaryViewPosition_Field2 = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletBarcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletBarcode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletBarcode[] = L"Windows.ApplicationModel.Wallet.IWalletBarcode";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Symbology)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletBarcodeSymbology* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetImageAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamReference** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_get_Symbology(This, value) \
    ((This)->lpVtbl->get_Symbology(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_GetImageAsync(This, operation) \
    ((This)->lpVtbl->GetImageAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletBarcodeFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletBarcode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletBarcodeFactory[] = L"Windows.ApplicationModel.Wallet.IWalletBarcodeFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* CreateWalletBarcode)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletBarcodeSymbology symbology,
        HSTRING value,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode** barcode);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* CreateCustomWalletBarcode)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* streamToBarcodeImage,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode** barcode);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_CreateWalletBarcode(This, symbology, value, barcode) \
    ((This)->lpVtbl->CreateWalletBarcode(This, symbology, value, barcode))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletBarcodeFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_CreateCustomWalletBarcode(This, streamToBarcodeImage, barcode) \
    ((This)->lpVtbl->CreateCustomWalletBarcode(This, streamToBarcodeImage, barcode))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcodeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItem[] = L"Windows.ApplicationModel.Wallet.IWalletItem";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IsAcknowledged)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IsAcknowledged)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IssuerDisplayName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IssuerDisplayName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_LastUpdated)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_LastUpdated)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Barcode)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Barcode)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletBarcode* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_ExpirationDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_ExpirationDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Logo159x159)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Logo159x159)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Logo336x336)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Logo336x336)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Logo99x99)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Logo99x99)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IsDisplayMessageLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IsDisplayMessageLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_LogoText)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_LogoText)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_HeaderColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_HeaderColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_BodyColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_BodyColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_HeaderFontColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_HeaderFontColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_BodyFontColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_BodyFontColor)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        struct __x_ABI_CWindows_CUI_CColor value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_HeaderBackgroundImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_HeaderBackgroundImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_BodyBackgroundImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_BodyBackgroundImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_LogoImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_LogoImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_PromotionalImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_PromotionalImage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_RelevantDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_RelevantDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_RelevantDateDisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_RelevantDateDisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_TransactionHistory)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletTransaction** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_RelevantLocations)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletRelevantLocation** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IsMoreTransactionHistoryLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IsMoreTransactionHistoryLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayProperties)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletItemCustomProperty** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Verbs)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* This,
        __FIMap_2_HSTRING_Windows__CApplicationModel__CWallet__CWalletVerb** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_IsAcknowledged(This, value) \
    ((This)->lpVtbl->get_IsAcknowledged(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_IsAcknowledged(This, value) \
    ((This)->lpVtbl->put_IsAcknowledged(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_IssuerDisplayName(This, value) \
    ((This)->lpVtbl->get_IssuerDisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_IssuerDisplayName(This, value) \
    ((This)->lpVtbl->put_IssuerDisplayName(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_LastUpdated(This, value) \
    ((This)->lpVtbl->get_LastUpdated(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_LastUpdated(This, value) \
    ((This)->lpVtbl->put_LastUpdated(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Barcode(This, value) \
    ((This)->lpVtbl->get_Barcode(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_Barcode(This, value) \
    ((This)->lpVtbl->put_Barcode(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_ExpirationDate(This, value) \
    ((This)->lpVtbl->get_ExpirationDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_ExpirationDate(This, value) \
    ((This)->lpVtbl->put_ExpirationDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Logo159x159(This, value) \
    ((This)->lpVtbl->get_Logo159x159(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_Logo159x159(This, value) \
    ((This)->lpVtbl->put_Logo159x159(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Logo336x336(This, value) \
    ((This)->lpVtbl->get_Logo336x336(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_Logo336x336(This, value) \
    ((This)->lpVtbl->put_Logo336x336(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Logo99x99(This, value) \
    ((This)->lpVtbl->get_Logo99x99(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_Logo99x99(This, value) \
    ((This)->lpVtbl->put_Logo99x99(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_DisplayMessage(This, value) \
    ((This)->lpVtbl->get_DisplayMessage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_DisplayMessage(This, value) \
    ((This)->lpVtbl->put_DisplayMessage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_IsDisplayMessageLaunchable(This, value) \
    ((This)->lpVtbl->get_IsDisplayMessageLaunchable(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_IsDisplayMessageLaunchable(This, value) \
    ((This)->lpVtbl->put_IsDisplayMessageLaunchable(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_LogoText(This, value) \
    ((This)->lpVtbl->get_LogoText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_LogoText(This, value) \
    ((This)->lpVtbl->put_LogoText(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_HeaderColor(This, value) \
    ((This)->lpVtbl->get_HeaderColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_HeaderColor(This, value) \
    ((This)->lpVtbl->put_HeaderColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_BodyColor(This, value) \
    ((This)->lpVtbl->get_BodyColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_BodyColor(This, value) \
    ((This)->lpVtbl->put_BodyColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_HeaderFontColor(This, value) \
    ((This)->lpVtbl->get_HeaderFontColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_HeaderFontColor(This, value) \
    ((This)->lpVtbl->put_HeaderFontColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_BodyFontColor(This, value) \
    ((This)->lpVtbl->get_BodyFontColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_BodyFontColor(This, value) \
    ((This)->lpVtbl->put_BodyFontColor(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_HeaderBackgroundImage(This, value) \
    ((This)->lpVtbl->get_HeaderBackgroundImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_HeaderBackgroundImage(This, value) \
    ((This)->lpVtbl->put_HeaderBackgroundImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_BodyBackgroundImage(This, value) \
    ((This)->lpVtbl->get_BodyBackgroundImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_BodyBackgroundImage(This, value) \
    ((This)->lpVtbl->put_BodyBackgroundImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_LogoImage(This, value) \
    ((This)->lpVtbl->get_LogoImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_LogoImage(This, value) \
    ((This)->lpVtbl->put_LogoImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_PromotionalImage(This, value) \
    ((This)->lpVtbl->get_PromotionalImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_PromotionalImage(This, value) \
    ((This)->lpVtbl->put_PromotionalImage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_RelevantDate(This, value) \
    ((This)->lpVtbl->get_RelevantDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_RelevantDate(This, value) \
    ((This)->lpVtbl->put_RelevantDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_RelevantDateDisplayMessage(This, value) \
    ((This)->lpVtbl->get_RelevantDateDisplayMessage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_RelevantDateDisplayMessage(This, value) \
    ((This)->lpVtbl->put_RelevantDateDisplayMessage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_TransactionHistory(This, value) \
    ((This)->lpVtbl->get_TransactionHistory(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_RelevantLocations(This, value) \
    ((This)->lpVtbl->get_RelevantLocations(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_IsMoreTransactionHistoryLaunchable(This, value) \
    ((This)->lpVtbl->get_IsMoreTransactionHistoryLaunchable(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_put_IsMoreTransactionHistoryLaunchable(This, value) \
    ((This)->lpVtbl->put_IsMoreTransactionHistoryLaunchable(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_DisplayProperties(This, value) \
    ((This)->lpVtbl->get_DisplayProperties(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_get_Verbs(This, value) \
    ((This)->lpVtbl->get_Verbs(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemCustomProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemCustomProperty[] = L"Windows.ApplicationModel.Wallet.IWalletItemCustomProperty";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_AutoDetectLinks)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_AutoDetectLinks)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DetailViewPosition)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletDetailViewPosition* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DetailViewPosition)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletDetailViewPosition value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_SummaryViewPosition)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletSummaryViewPosition* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_SummaryViewPosition)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletSummaryViewPosition value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_get_AutoDetectLinks(This, value) \
    ((This)->lpVtbl->get_AutoDetectLinks(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_put_AutoDetectLinks(This, value) \
    ((This)->lpVtbl->put_AutoDetectLinks(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_get_DetailViewPosition(This, value) \
    ((This)->lpVtbl->get_DetailViewPosition(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_put_DetailViewPosition(This, value) \
    ((This)->lpVtbl->put_DetailViewPosition(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_get_SummaryViewPosition(This, value) \
    ((This)->lpVtbl->get_SummaryViewPosition(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_put_SummaryViewPosition(This, value) \
    ((This)->lpVtbl->put_SummaryViewPosition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemCustomPropertyFactory[] = L"Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* CreateWalletItemCustomProperty)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory* This,
        HSTRING name,
        HSTRING value,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomProperty** walletItemCustomProperty);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_CreateWalletItemCustomProperty(This, name, value, walletItemCustomProperty) \
    ((This)->lpVtbl->CreateWalletItemCustomProperty(This, name, value, walletItemCustomProperty))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemCustomPropertyFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemFactory[] = L"Windows.ApplicationModel.Wallet.IWalletItemFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* CreateWalletItem)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind kind,
        HSTRING displayName,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** walletItem);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_CreateWalletItem(This, kind, displayName, walletItem) \
    ((This)->lpVtbl->CreateWalletItem(This, kind, displayName, walletItem))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemStore[] = L"Windows.ApplicationModel.Wallet.IWalletItemStore";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* AddAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        HSTRING id,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* item,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* ClearAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetWalletItemAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        HSTRING id,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetItemsAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetItemsWithKindAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CWalletItemKind kind,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* ImportItemAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* stream,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        HSTRING id,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* ShowAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* ShowItemAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        HSTRING id,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* UpdateAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* item,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_AddAsync(This, id, item, operation) \
    ((This)->lpVtbl->AddAsync(This, id, item, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_ClearAsync(This, operation) \
    ((This)->lpVtbl->ClearAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetWalletItemAsync(This, id, operation) \
    ((This)->lpVtbl->GetWalletItemAsync(This, id, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetItemsAsync(This, operation) \
    ((This)->lpVtbl->GetItemsAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_GetItemsWithKindAsync(This, kind, operation) \
    ((This)->lpVtbl->GetItemsWithKindAsync(This, kind, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_ImportItemAsync(This, stream, operation) \
    ((This)->lpVtbl->ImportItemAsync(This, stream, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_DeleteAsync(This, id, operation) \
    ((This)->lpVtbl->DeleteAsync(This, id, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_ShowAsync(This, operation) \
    ((This)->lpVtbl->ShowAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_ShowItemAsync(This, id, operation) \
    ((This)->lpVtbl->ShowItemAsync(This, id, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_UpdateAsync(This, item, operation) \
    ((This)->lpVtbl->UpdateAsync(This, item, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletItemStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletItemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletItemStore2[] = L"Windows.ApplicationModel.Wallet.IWalletItemStore2";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* add_ItemsChanged)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CWalletItemStore_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* remove_ItemsChanged)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_add_ItemsChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_ItemsChanged(This, handler, cookie))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_remove_ItemsChanged(This, cookie) \
    ((This)->lpVtbl->remove_ItemsChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItemStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletManagerStatics[] = L"Windows.ApplicationModel.Wallet.IWalletManagerStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemStore** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletManagerStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_RequestStoreAsync(This, operation) \
    ((This)->lpVtbl->RequestStoreAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletRelevantLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletRelevantLocation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletRelevantLocation[] = L"Windows.ApplicationModel.Wallet.IWalletRelevantLocation";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        struct __x_ABI_CWindows_CDevices_CGeolocation_CBasicGeoposition value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DisplayMessage)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_get_DisplayMessage(This, value) \
    ((This)->lpVtbl->get_DisplayMessage(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_put_DisplayMessage(This, value) \
    ((This)->lpVtbl->put_DisplayMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletRelevantLocation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletTransaction[] = L"Windows.ApplicationModel.Wallet.IWalletTransaction";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayAmount)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DisplayAmount)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IgnoreTimeOfDay)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IgnoreTimeOfDay)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        boolean value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_DisplayLocation)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_DisplayLocation)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_TransactionDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_TransactionDate)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_IsLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_IsLaunchable)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransactionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_DisplayAmount(This, value) \
    ((This)->lpVtbl->get_DisplayAmount(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_DisplayAmount(This, value) \
    ((This)->lpVtbl->put_DisplayAmount(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_IgnoreTimeOfDay(This, value) \
    ((This)->lpVtbl->get_IgnoreTimeOfDay(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_IgnoreTimeOfDay(This, value) \
    ((This)->lpVtbl->put_IgnoreTimeOfDay(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_DisplayLocation(This, value) \
    ((This)->lpVtbl->get_DisplayLocation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_DisplayLocation(This, value) \
    ((This)->lpVtbl->put_DisplayLocation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_TransactionDate(This, value) \
    ((This)->lpVtbl->get_TransactionDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_TransactionDate(This, value) \
    ((This)->lpVtbl->put_TransactionDate(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_get_IsLaunchable(This, value) \
    ((This)->lpVtbl->get_IsLaunchable(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_put_IsLaunchable(This, value) \
    ((This)->lpVtbl->put_IsLaunchable(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletVerb
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletVerb
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletVerb[] = L"Windows.ApplicationModel.Wallet.IWalletVerb";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.IWalletVerbFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.WalletVerb
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_IWalletVerbFactory[] = L"Windows.ApplicationModel.Wallet.IWalletVerbFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletVerbFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerbFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* CreateWalletVerb)(__x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerb** WalletVerb);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletVerbFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_CreateWalletVerb(This, name, WalletVerb) \
    ((This)->lpVtbl->CreateWalletVerb(This, name, WalletVerb))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletVerbFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletBarcode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletBarcodeFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletBarcode ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletBarcode_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletBarcode_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletBarcode is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletBarcode[] = L"Windows.ApplicationModel.Wallet.WalletBarcode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletItemFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItem_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItem[] = L"Windows.ApplicationModel.Wallet.WalletItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItemCustomProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletItemCustomPropertyFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItemCustomProperty ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemCustomProperty_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemCustomProperty_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemCustomProperty is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItemCustomProperty[] = L"Windows.ApplicationModel.Wallet.WalletItemCustomProperty";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletItemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletItemStore ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletItemStore_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletItemStore[] = L"Windows.ApplicationModel.Wallet.WalletItemStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Wallet.IWalletManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletManager_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletManager is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletManager[] = L"Windows.ApplicationModel.Wallet.WalletManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletRelevantLocation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletRelevantLocation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletRelevantLocation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletRelevantLocation_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletRelevantLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletRelevantLocation[] = L"Windows.ApplicationModel.Wallet.WalletRelevantLocation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletTransaction ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletTransaction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletTransaction_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletTransaction is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletTransaction[] = L"Windows.ApplicationModel.Wallet.WalletTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.WalletVerb
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Wallet.IWalletVerbFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.IWalletVerb ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletVerb_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_WalletVerb_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletVerb is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_WalletVerb[] = L"Windows.ApplicationModel.Wallet.WalletVerb";
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
#endif // __windows2Eapplicationmodel2Ewallet_p_h__

#endif // __windows2Eapplicationmodel2Ewallet_h__
