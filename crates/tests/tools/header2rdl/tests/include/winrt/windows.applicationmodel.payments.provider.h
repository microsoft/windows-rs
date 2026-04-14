
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
#ifndef __windows2Eapplicationmodel2Epayments2Eprovider_h__
#define __windows2Eapplicationmodel2Epayments2Eprovider_h__
#ifndef __windows2Eapplicationmodel2Epayments2Eprovider_p_h__
#define __windows2Eapplicationmodel2Epayments2Eprovider_p_h__


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
#include "Windows.ApplicationModel.Payments.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentAppCanMakePaymentTriggerDetails;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails ABI::Windows::ApplicationModel::Payments::Provider::IPaymentAppCanMakePaymentTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentAppManager;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager ABI::Windows::ApplicationModel::Payments::Provider::IPaymentAppManager

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentAppManagerStatics;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics ABI::Windows::ApplicationModel::Payments::Provider::IPaymentAppManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentTransaction;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransaction

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentTransactionAcceptResult;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransactionAcceptResult

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    interface IPaymentTransactionStatics;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransactionStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequestChangedResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestChangedResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0cc32025-ac67-57e2-a0f6-3a8e116cef4c"))
IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*, ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Payments.PaymentRequestChangedResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bec8b726-9056-5e47-b22a-0da09aa84afe"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*, ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Payments.PaymentRequestChangedResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentRequestChangedResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    class PaymentTransaction;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e09a3f7d-6ad0-58cf-ab4c-2e4c9c791873"))
IAsyncOperation<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*, ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Payments.Provider.PaymentTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*> __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd5b92e5-1086-5c3d-9de1-9982e776d193"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*, ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransaction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Payments.Provider.PaymentTransaction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransaction*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    class PaymentTransactionAcceptResult;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c645e8ef-0863-55b4-8aed-42bd152d8004"))
IAsyncOperation<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*, ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransactionAcceptResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4538f88a-89af-50b9-8165-7e6269639884"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*, ABI::Windows::ApplicationModel::Payments::Provider::IPaymentTransactionAcceptResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::Provider::PaymentTransactionAcceptResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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
            namespace Payments {
                class PaymentAddress;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentAddress;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress ABI::Windows::ApplicationModel::Payments::IPaymentAddress

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentCanMakePaymentResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentCanMakePaymentResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResult

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequest;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequest;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest ABI::Windows::ApplicationModel::Payments::IPaymentRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                typedef enum PaymentRequestCompletionStatus : int PaymentRequestCompletionStatus;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentShippingOption;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentShippingOption;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentToken;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentToken;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken ABI::Windows::ApplicationModel::Payments::IPaymentToken

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__

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
            namespace Payments {
                namespace Provider {
                    class PaymentAppManager;
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppCanMakePaymentTriggerDetails[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6")
                    IPaymentAppCanMakePaymentTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Payments::IPaymentRequest** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCanMakePaymentResult(
                            ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResult* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentAppCanMakePaymentTriggerDetails = __uuidof(IPaymentAppCanMakePaymentTriggerDetails);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppManager[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("0e47aa53-8521-4969-a957-df2538a3a98f")
                    IPaymentAppManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RegisterAsync(
                            __FIIterable_1_HSTRING* supportedPaymentMethodIds,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UnregisterAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentAppManager = __uuidof(IPaymentAppManager);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppManagerStatics[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("a341ac28-fc89-4406-b4d9-34e7fe79dfb6")
                    IPaymentAppManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::ApplicationModel::Payments::Provider::IPaymentAppManager** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentAppManagerStatics = __uuidof(IPaymentAppManagerStatics);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransaction[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransaction";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("62581da0-26a5-4e9b-a6eb-66606cf001d3")
                    IPaymentTransaction : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PaymentRequest(
                            ABI::Windows::ApplicationModel::Payments::IPaymentRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PayerEmail(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PayerEmail(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PayerName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PayerName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PayerPhoneNumber(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PayerPhoneNumber(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateShippingAddressAsync(
                            ABI::Windows::ApplicationModel::Payments::IPaymentAddress* shippingAddress,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateSelectedShippingOptionAsync(
                            ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption* selectedShippingOption,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AcceptAsync(
                            ABI::Windows::ApplicationModel::Payments::IPaymentToken* paymentToken,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Reject(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentTransaction = __uuidof(IPaymentTransaction);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransactionAcceptResult[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("060e3276-d30c-4817-95a2-df7ae9273b56")
                    IPaymentTransactionAcceptResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::ApplicationModel::Payments::PaymentRequestCompletionStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentTransactionAcceptResult = __uuidof(IPaymentTransactionAcceptResult);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransactionStatics[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                namespace Provider {
                    MIDL_INTERFACE("8d639750-ee0a-4df5-9b1e-1c0f9ec59881")
                    IPaymentTransactionStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING id,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaymentTransactionStatics = __uuidof(IPaymentTransactionStatics);
                } /* Provider */
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails[] = L"Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentAppManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentAppManager[] = L"Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentTransaction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransaction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransaction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentTransaction[] = L"Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult[] = L"Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus;

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppCanMakePaymentTriggerDetails[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** result);
    HRESULT (STDMETHODCALLTYPE* ReportCanMakePaymentResult)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_get_Request(This, result) \
    ((This)->lpVtbl->get_Request(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_ReportCanMakePaymentResult(This, value) \
    ((This)->lpVtbl->ReportCanMakePaymentResult(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppCanMakePaymentTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppManager[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        __FIIterable_1_HSTRING* supportedPaymentMethodIds,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* UnregisterAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_RegisterAsync(This, supportedPaymentMethodIds, result) \
    ((This)->lpVtbl->RegisterAsync(This, supportedPaymentMethodIds, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_UnregisterAsync(This, result) \
    ((This)->lpVtbl->UnregisterAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentAppManagerStatics[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentAppManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransaction[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransaction";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PaymentRequest)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** value);
    HRESULT (STDMETHODCALLTYPE* get_PayerEmail)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PayerEmail)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PayerName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PayerName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PayerPhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PayerPhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* UpdateShippingAddressAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* shippingAddress,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult** result);
    HRESULT (STDMETHODCALLTYPE* UpdateSelectedShippingOptionAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* selectedShippingOption,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestChangedResult** result);
    HRESULT (STDMETHODCALLTYPE* AcceptAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* paymentToken,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransactionAcceptResult** result);
    HRESULT (STDMETHODCALLTYPE* Reject)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_get_PaymentRequest(This, value) \
    ((This)->lpVtbl->get_PaymentRequest(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_get_PayerEmail(This, value) \
    ((This)->lpVtbl->get_PayerEmail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_put_PayerEmail(This, value) \
    ((This)->lpVtbl->put_PayerEmail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_get_PayerName(This, value) \
    ((This)->lpVtbl->get_PayerName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_put_PayerName(This, value) \
    ((This)->lpVtbl->put_PayerName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_get_PayerPhoneNumber(This, value) \
    ((This)->lpVtbl->get_PayerPhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_put_PayerPhoneNumber(This, value) \
    ((This)->lpVtbl->put_PayerPhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_UpdateShippingAddressAsync(This, shippingAddress, result) \
    ((This)->lpVtbl->UpdateShippingAddressAsync(This, shippingAddress, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_UpdateSelectedShippingOptionAsync(This, selectedShippingOption, result) \
    ((This)->lpVtbl->UpdateSelectedShippingOptionAsync(This, selectedShippingOption, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_AcceptAsync(This, paymentToken, result) \
    ((This)->lpVtbl->AcceptAsync(This, paymentToken, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_Reject(This) \
    ((This)->lpVtbl->Reject(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransaction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransactionAcceptResult[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionAcceptResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_Provider_IPaymentTransactionStatics[] = L"Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics* This,
        HSTRING id,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CProvider__CPaymentTransaction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_FromIdAsync(This, id, result) \
    ((This)->lpVtbl->FromIdAsync(This, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CProvider_CIPaymentTransactionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentAppCanMakePaymentTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentAppCanMakePaymentTriggerDetails[] = L"Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentAppManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Payments.Provider.IPaymentAppManagerStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentAppManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentAppManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentAppManager[] = L"Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentTransaction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Payments.Provider.IPaymentTransactionStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentTransaction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransaction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransaction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentTransaction[] = L"Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.Provider.IPaymentTransactionAcceptResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_Provider_PaymentTransactionAcceptResult[] = L"Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Epayments2Eprovider_p_h__

#endif // __windows2Eapplicationmodel2Epayments2Eprovider_h__
