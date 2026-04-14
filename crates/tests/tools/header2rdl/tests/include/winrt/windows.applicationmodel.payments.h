
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
#ifndef __windows2Eapplicationmodel2Epayments_h__
#define __windows2Eapplicationmodel2Epayments_h__
#ifndef __windows2Eapplicationmodel2Epayments_p_h__
#define __windows2Eapplicationmodel2Epayments_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestChangedHandler;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedHandler

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentCanMakePaymentResultFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResultFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentCurrencyAmount;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentCurrencyAmountFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmountFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentDetails;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails ABI::Windows::ApplicationModel::Payments::IPaymentDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentDetailsFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory ABI::Windows::ApplicationModel::Payments::IPaymentDetailsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentDetailsModifier;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentDetailsModifierFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifierFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentItem;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem ABI::Windows::ApplicationModel::Payments::IPaymentItem

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentItemFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory ABI::Windows::ApplicationModel::Payments::IPaymentItemFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMediator;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator ABI::Windows::ApplicationModel::Payments::IPaymentMediator

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMediator2;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2 ABI::Windows::ApplicationModel::Payments::IPaymentMediator2

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMerchantInfo;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMerchantInfoFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfoFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMethodData;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData ABI::Windows::ApplicationModel::Payments::IPaymentMethodData

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentMethodDataFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory ABI::Windows::ApplicationModel::Payments::IPaymentMethodDataFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentOptions;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions ABI::Windows::ApplicationModel::Payments::IPaymentOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequest2;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2 ABI::Windows::ApplicationModel::Payments::IPaymentRequest2

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestChangedArgs;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestChangedResultFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResultFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory ABI::Windows::ApplicationModel::Payments::IPaymentRequestFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestFactory2;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2 ABI::Windows::ApplicationModel::Payments::IPaymentRequestFactory2

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentRequestSubmitResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult ABI::Windows::ApplicationModel::Payments::IPaymentRequestSubmitResult

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentResponse;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse ABI::Windows::ApplicationModel::Payments::IPaymentResponse

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentShippingOptionFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory ABI::Windows::ApplicationModel::Payments::IPaymentShippingOptionFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                interface IPaymentTokenFactory;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory ABI::Windows::ApplicationModel::Payments::IPaymentTokenFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentCanMakePaymentResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a467410a-11de-5090-b905-96a562d85de5"))
IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*, ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89ae5b89-6d05-5842-9cdf-f4cbf706dc5e"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*, ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequestSubmitResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cf290deb-5549-57c3-8abd-53b76c643cca"))
IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*, ABI::Windows::ApplicationModel::Payments::IPaymentRequestSubmitResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Payments.PaymentRequestSubmitResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cbcd07a6-ae2a-5a70-bc0b-9120560825d1"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*, ABI::Windows::ApplicationModel::Payments::IPaymentRequestSubmitResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Payments.PaymentRequestSubmitResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Payments::PaymentRequestSubmitResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_USE */

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



#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f92b529-119b-575a-a419-3904b4e41af2"))
IAsyncOperation<__FIVectorView_1_HSTRING*> : IAsyncOperation_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_HSTRING*> __FIAsyncOperation_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c7899be-5f2e-5bf3-ade5-ad98b772c7cd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentDetailsModifier;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fafb6774-b665-5b8b-b1ef-95038c3aabe1"))
IIterator<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*, ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Payments.PaymentDetailsModifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t;
#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("585d2b3d-cb34-58c4-81f4-1ea157996def"))
IIterable<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*, ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Payments.PaymentDetailsModifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t;
#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentItem;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93c12cc3-6a0b-5f02-ac74-056007472731"))
IIterator<ABI::Windows::ApplicationModel::Payments::PaymentItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentItem*, ABI::Windows::ApplicationModel::Payments::IPaymentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Payments.PaymentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Payments::PaymentItem*> __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b61b704c-e5fa-5524-8b95-7d03f5d36ae9"))
IIterable<ABI::Windows::ApplicationModel::Payments::PaymentItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentItem*, ABI::Windows::ApplicationModel::Payments::IPaymentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Payments.PaymentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Payments::PaymentItem*> __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentMethodData;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9666522e-e5ac-5374-a5d9-5cf57c4bf689"))
IIterator<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*, ABI::Windows::ApplicationModel::Payments::IPaymentMethodData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Payments.PaymentMethodData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t;
#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c9ed87e-4add-58a8-ad9a-9dfa48ca250a"))
IIterable<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*, ABI::Windows::ApplicationModel::Payments::IPaymentMethodData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Payments.PaymentMethodData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t;
#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentShippingOption;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("49edc9f4-2ce6-534c-b529-5ceec705def5"))
IIterator<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*, ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Payments.PaymentShippingOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t;
#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de881c69-6642-54de-a8f7-d1a88b2404cf"))
IIterable<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*, ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Payments.PaymentShippingOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t;
#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("da7e871c-6e0c-5e32-be40-10715a9f75eb"))
IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*, ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Payments.PaymentDetailsModifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentDetailsModifier*> __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6baf1241-1fa6-5c22-83ef-415e93cdf7b1"))
IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentItem*, ABI::Windows::ApplicationModel::Payments::IPaymentItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Payments.PaymentItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentItem*> __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1011b9ba-6a05-5b16-82cf-0175085105e0"))
IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*, ABI::Windows::ApplicationModel::Payments::IPaymentMethodData*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Payments.PaymentMethodData>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentMethodData*> __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c8128eb-8c33-56dd-9648-39e8040312d4"))
IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*, ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Payments.PaymentShippingOption>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Payments::PaymentShippingOption*> __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

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
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                typedef enum PaymentCanMakePaymentResultStatus : int PaymentCanMakePaymentResultStatus;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                typedef enum PaymentOptionPresence : int PaymentOptionPresence;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                typedef enum PaymentRequestChangeKind : int PaymentRequestChangeKind;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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
                typedef enum PaymentRequestStatus : int PaymentRequestStatus;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                typedef enum PaymentShippingType : int PaymentShippingType;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentAddress;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentCurrencyAmount;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentDetails;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentMerchantInfo;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentOptions;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequest;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequestChangedArgs;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentRequestChangedResult;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentResponse;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                class PaymentToken;
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentCanMakePaymentResultStatus : int
                {
                    PaymentCanMakePaymentResultStatus_Unknown = 0,
                    PaymentCanMakePaymentResultStatus_Yes = 1,
                    PaymentCanMakePaymentResultStatus_No = 2,
                    PaymentCanMakePaymentResultStatus_NotAllowed = 3,
                    PaymentCanMakePaymentResultStatus_UserNotSignedIn = 4,
                    PaymentCanMakePaymentResultStatus_SpecifiedPaymentMethodIdsNotSupported = 5,
                    PaymentCanMakePaymentResultStatus_NoQualifyingCardOnFile = 6,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentOptionPresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentOptionPresence : int
                {
                    PaymentOptionPresence_None = 0,
                    PaymentOptionPresence_Optional = 1,
                    PaymentOptionPresence_Required = 2,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentRequestChangeKind : int
                {
                    PaymentRequestChangeKind_ShippingOption = 0,
                    PaymentRequestChangeKind_ShippingAddress = 1,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentRequestCompletionStatus : int
                {
                    PaymentRequestCompletionStatus_Succeeded = 0,
                    PaymentRequestCompletionStatus_Failed = 1,
                    PaymentRequestCompletionStatus_Unknown = 2,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentRequestStatus : int
                {
                    PaymentRequestStatus_Succeeded = 0,
                    PaymentRequestStatus_Failed = 1,
                    PaymentRequestStatus_Canceled = 2,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentShippingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                enum PaymentShippingType : int
                {
                    PaymentShippingType_Shipping = 0,
                    PaymentShippingType_Delivery = 1,
                    PaymentShippingType_Pickup = 2,
                };
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Delegate Windows.ApplicationModel.Payments.PaymentRequestChangedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("5078b9e1-f398-4f2c-a27e-94d371cf6c7d")
                IPaymentRequestChangedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest* paymentRequest,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedArgs* args
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestChangedHandler = __uuidof(IPaymentRequestChangedHandler);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentAddress[] = L"Windows.ApplicationModel.Payments.IPaymentAddress";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("5f2264e9-6f3a-4166-a018-0a0b06bb32b5")
                IPaymentAddress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Country(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Country(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AddressLines(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AddressLines(
                        __FIVectorView_1_HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Region(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Region(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_City(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_City(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DependentLocality(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DependentLocality(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PostalCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PostalCode(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SortingCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SortingCode(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LanguageCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LanguageCode(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Organization(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Organization(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Recipient(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Recipient(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PhoneNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentAddress = __uuidof(IPaymentAddress);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCanMakePaymentResult[] = L"Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("7696fe55-d5d3-4d3d-b345-45591759c510")
                IPaymentCanMakePaymentResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResultStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentCanMakePaymentResult = __uuidof(IPaymentCanMakePaymentResult);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCanMakePaymentResultFactory[] = L"Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("bbdcaa3e-7d49-4f69-aa53-2a0f8164b7c9")
                IPaymentCanMakePaymentResultFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::ApplicationModel::Payments::PaymentCanMakePaymentResultStatus value,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCanMakePaymentResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentCanMakePaymentResultFactory = __uuidof(IPaymentCanMakePaymentResultFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCurrencyAmount[] = L"Windows.ApplicationModel.Payments.IPaymentCurrencyAmount";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("e3a3e9e0-b41f-4987-bdcb-071331f2daa4")
                IPaymentCurrencyAmount : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Currency(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Currency(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrencySystem(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CurrencySystem(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentCurrencyAmount = __uuidof(IPaymentCurrencyAmount);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCurrencyAmountFactory[] = L"Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("3257d338-140c-4575-8535-f773178c09a7")
                IPaymentCurrencyAmountFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING value,
                        HSTRING currency,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithCurrencySystem(
                        HSTRING value,
                        HSTRING currency,
                        HSTRING currencySystem,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentCurrencyAmountFactory = __uuidof(IPaymentCurrencyAmountFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetails[] = L"Windows.ApplicationModel.Payments.IPaymentDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("53bb2d7d-e0eb-4053-8eae-ce7c48e02945")
                IPaymentDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Total(
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Total(
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayItems(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayItems(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShippingOptions(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShippingOptions(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Modifiers(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Modifiers(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentDetails = __uuidof(IPaymentDetails);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsFactory[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("cfe8afee-c0ea-4ca1-8bc7-6de67b1f3763")
                IPaymentDetailsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* total,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithDisplayItems(
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* total,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* displayItems,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentDetailsFactory = __uuidof(IPaymentDetailsFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsModifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsModifier[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsModifier";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("be1c7d65-4323-41d7-b305-dfcb765f69de")
                IPaymentDetailsModifier : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_JsonData(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedMethodIds(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Total(
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdditionalDisplayItems(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentDetailsModifier = __uuidof(IPaymentDetailsModifier);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsModifierFactory[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("79005286-54de-429c-9e4f-5dce6e10ebce")
                IPaymentDetailsModifierFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_HSTRING* supportedMethodIds,
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* total,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAdditionalDisplayItems(
                        __FIIterable_1_HSTRING* supportedMethodIds,
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* total,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* additionalDisplayItems,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithAdditionalDisplayItemsAndJsonData(
                        __FIIterable_1_HSTRING* supportedMethodIds,
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem* total,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* additionalDisplayItems,
                        HSTRING jsonData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetailsModifier** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentDetailsModifierFactory = __uuidof(IPaymentDetailsModifierFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentItem[] = L"Windows.ApplicationModel.Payments.IPaymentItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("685ac88b-79b2-4b76-9e03-a876223dfe72")
                IPaymentItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Label(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Amount(
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Amount(
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pending(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Pending(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentItem = __uuidof(IPaymentItem);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentItemFactory[] = L"Windows.ApplicationModel.Payments.IPaymentItemFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("c6ab7ad8-2503-4d1d-a778-02b2e5927b2c")
                IPaymentItemFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING label,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* amount,
                        ABI::Windows::ApplicationModel::Payments::IPaymentItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentItemFactory = __uuidof(IPaymentItemFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMediator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMediator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMediator[] = L"Windows.ApplicationModel.Payments.IPaymentMediator";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("fb0ee829-ec0c-449a-83da-7ae3073365a2")
                IPaymentMediator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedMethodIdsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SubmitPaymentRequestAsync(
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest* paymentRequest,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SubmitPaymentRequestWithChangeHandlerAsync(
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest* paymentRequest,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedHandler* changeHandler,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMediator = __uuidof(IPaymentMediator);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMediator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMediator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMediator2[] = L"Windows.ApplicationModel.Payments.IPaymentMediator2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("ceef98f1-e407-4128-8e73-d93d5f822786")
                IPaymentMediator2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CanMakePaymentAsync(
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest* paymentRequest,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMediator2 = __uuidof(IPaymentMediator2);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMerchantInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMerchantInfo[] = L"Windows.ApplicationModel.Payments.IPaymentMerchantInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("63445050-0e94-4ed6-aacb-e6012bd327a7")
                IPaymentMerchantInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFullName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMerchantInfo = __uuidof(IPaymentMerchantInfo);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMerchantInfoFactory[] = L"Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("9e89ced3-ccb7-4167-a8ec-e10ae96dbcd1")
                IPaymentMerchantInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMerchantInfoFactory = __uuidof(IPaymentMerchantInfoFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMethodData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMethodData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMethodData[] = L"Windows.ApplicationModel.Payments.IPaymentMethodData";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("d1d3caf4-de98-4129-b1b7-c3ad86237bf4")
                IPaymentMethodData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedMethodIds(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JsonData(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMethodData = __uuidof(IPaymentMethodData);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMethodDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMethodData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMethodDataFactory[] = L"Windows.ApplicationModel.Payments.IPaymentMethodDataFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("8addd27f-9baa-4a82-8342-a8210992a36b")
                IPaymentMethodDataFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        __FIIterable_1_HSTRING* supportedMethodIds,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMethodData** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithJsonData(
                        __FIIterable_1_HSTRING* supportedMethodIds,
                        HSTRING jsonData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMethodData** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentMethodDataFactory = __uuidof(IPaymentMethodDataFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentOptions[] = L"Windows.ApplicationModel.Payments.IPaymentOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("aaa30854-1f2b-4365-8251-01b58915a5bc")
                IPaymentOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequestPayerEmail(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestPayerEmail(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestPayerName(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestPayerName(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestPayerPhoneNumber(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestPayerPhoneNumber(
                        ABI::Windows::ApplicationModel::Payments::PaymentOptionPresence value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestShipping(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestShipping(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShippingType(
                        ABI::Windows::ApplicationModel::Payments::PaymentShippingType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShippingType(
                        ABI::Windows::ApplicationModel::Payments::PaymentShippingType value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentOptions = __uuidof(IPaymentOptions);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequest[] = L"Windows.ApplicationModel.Payments.IPaymentRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("b74942e1-ed7b-47eb-bc08-78cc5d6896b6")
                IPaymentRequest : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MerchantInfo(
                        ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MethodData(
                        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        ABI::Windows::ApplicationModel::Payments::IPaymentOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequest = __uuidof(IPaymentRequest);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequest2[] = L"Windows.ApplicationModel.Payments.IPaymentRequest2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("b63ccfb5-5998-493e-a04c-67048a50f141")
                IPaymentRequest2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequest2 = __uuidof(IPaymentRequest2);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedArgs[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("c6145e44-cd8b-4be4-b555-27c99194c0c5")
                IPaymentRequestChangedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeKind(
                        ABI::Windows::ApplicationModel::Payments::PaymentRequestChangeKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShippingAddress(
                        ABI::Windows::ApplicationModel::Payments::IPaymentAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedShippingOption(
                        ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Acknowledge(
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult* changeResult
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestChangedArgs = __uuidof(IPaymentRequestChangedArgs);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedResult[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("df699e5c-16c4-47ad-9401-8440ec0757db")
                IPaymentRequestChangedResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeAcceptedByMerchant(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ChangeAcceptedByMerchant(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Message(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdatedPaymentDetails(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UpdatedPaymentDetails(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestChangedResult = __uuidof(IPaymentRequestChangedResult);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedResultFactory[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("08740f56-1d33-4431-814b-67ea24bf21db")
                IPaymentRequestChangedResultFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        boolean changeAcceptedByMerchant,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithPaymentDetails(
                        boolean changeAcceptedByMerchant,
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* updatedPaymentDetails,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequestChangedResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestChangedResultFactory = __uuidof(IPaymentRequestChangedResultFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestFactory[] = L"Windows.ApplicationModel.Payments.IPaymentRequestFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("3e8a79dc-6b74-42d3-b103-f0de35fb1848")
                IPaymentRequestFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* details,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithMerchantInfo(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* details,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo* merchantInfo,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithMerchantInfoAndOptions(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* details,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo* merchantInfo,
                        ABI::Windows::ApplicationModel::Payments::IPaymentOptions* options,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestFactory = __uuidof(IPaymentRequestFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestFactory2[] = L"Windows.ApplicationModel.Payments.IPaymentRequestFactory2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("e6ce1325-a506-4372-b7ef-1a031d5662d1")
                IPaymentRequestFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithMerchantInfoOptionsAndId(
                        ABI::Windows::ApplicationModel::Payments::IPaymentDetails* details,
                        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
                        ABI::Windows::ApplicationModel::Payments::IPaymentMerchantInfo* merchantInfo,
                        ABI::Windows::ApplicationModel::Payments::IPaymentOptions* options,
                        HSTRING id,
                        ABI::Windows::ApplicationModel::Payments::IPaymentRequest** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestFactory2 = __uuidof(IPaymentRequestFactory2);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestSubmitResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestSubmitResult[] = L"Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("7b9c3912-30f2-4e90-b249-8ce7d78ffe56")
                IPaymentRequestSubmitResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Payments::PaymentRequestStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Response(
                        ABI::Windows::ApplicationModel::Payments::IPaymentResponse** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentRequestSubmitResult = __uuidof(IPaymentRequestSubmitResult);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentResponse[] = L"Windows.ApplicationModel.Payments.IPaymentResponse";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("e1389457-8bd2-4888-9fa8-97985545108e")
                IPaymentResponse : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PaymentToken(
                        ABI::Windows::ApplicationModel::Payments::IPaymentToken** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShippingOption(
                        ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShippingAddress(
                        ABI::Windows::ApplicationModel::Payments::IPaymentAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PayerEmail(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PayerName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PayerPhoneNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CompleteAsync(
                        ABI::Windows::ApplicationModel::Payments::PaymentRequestCompletionStatus status,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentResponse = __uuidof(IPaymentResponse);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentShippingOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentShippingOption[] = L"Windows.ApplicationModel.Payments.IPaymentShippingOption";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("13372ada-9753-4574-8966-93145a76c7f9")
                IPaymentShippingOption : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Label(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Amount(
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Amount(
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSelected(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSelected(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentShippingOption = __uuidof(IPaymentShippingOption);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentShippingOptionFactory[] = L"Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("5de5f917-b2d7-446b-9d73-6123fbca3bc6")
                IPaymentShippingOptionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING label,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* amount,
                        ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSelected(
                        HSTRING label,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* amount,
                        boolean selected,
                        ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithSelectedAndTag(
                        HSTRING label,
                        ABI::Windows::ApplicationModel::Payments::IPaymentCurrencyAmount* amount,
                        boolean selected,
                        HSTRING tag,
                        ABI::Windows::ApplicationModel::Payments::IPaymentShippingOption** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentShippingOptionFactory = __uuidof(IPaymentShippingOptionFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentToken
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentToken[] = L"Windows.ApplicationModel.Payments.IPaymentToken";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("bbcac013-ccd0-41f2-b2a1-0a2e4b5dce25")
                IPaymentToken : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PaymentMethodId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JsonDetails(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentToken = __uuidof(IPaymentToken);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentTokenFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentToken
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentTokenFactory[] = L"Windows.ApplicationModel.Payments.IPaymentTokenFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Payments {
                MIDL_INTERFACE("988cd7aa-4753-4904-8373-dd7b08b995c1")
                IPaymentTokenFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING paymentMethodId,
                        ABI::Windows::ApplicationModel::Payments::IPaymentToken** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithJsonDetails(
                        HSTRING paymentMethodId,
                        HSTRING jsonDetails,
                        ABI::Windows::ApplicationModel::Payments::IPaymentToken** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPaymentTokenFactory = __uuidof(IPaymentTokenFactory);
            } /* Payments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentAddress_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentAddress[] = L"Windows.ApplicationModel.Payments.PaymentAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult[] = L"Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentCurrencyAmount ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCurrencyAmount_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCurrencyAmount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentCurrencyAmount[] = L"Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentDetailsFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentDetails[] = L"Windows.ApplicationModel.Payments.PaymentDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentDetailsModifier ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetailsModifier_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetailsModifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentDetailsModifier[] = L"Windows.ApplicationModel.Payments.PaymentDetailsModifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentItemFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentItem[] = L"Windows.ApplicationModel.Payments.PaymentItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMediator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMediator ** Default Interface **
 *    Windows.ApplicationModel.Payments.IPaymentMediator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMediator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMediator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMediator[] = L"Windows.ApplicationModel.Payments.PaymentMediator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMerchantInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMerchantInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMerchantInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMerchantInfo[] = L"Windows.ApplicationModel.Payments.PaymentMerchantInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMethodData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentMethodDataFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMethodData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMethodData_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMethodData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMethodData[] = L"Windows.ApplicationModel.Payments.PaymentMethodData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentOptions[] = L"Windows.ApplicationModel.Payments.PaymentOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestFactory2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequest ** Default Interface **
 *    Windows.ApplicationModel.Payments.IPaymentRequest2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequest[] = L"Windows.ApplicationModel.Payments.PaymentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs[] = L"Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestChangedResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestChangedResult[] = L"Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestSubmitResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult[] = L"Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentResponse[] = L"Windows.ApplicationModel.Payments.PaymentResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentShippingOption ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentShippingOption_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentShippingOption_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentShippingOption[] = L"Windows.ApplicationModel.Payments.PaymentShippingOption";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentTokenFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentToken ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentToken_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentToken_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentToken[] = L"Windows.ApplicationModel.Payments.PaymentToken";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2 __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2 __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2 __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult_INTERFACE_DEFINED__
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

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_HSTRING __FIAsyncOperation_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentMethodData** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        __FIIterator_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentCanMakePaymentResultStatus __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentCanMakePaymentResultStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence;

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestChangeKind __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestChangeKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestStatus __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentShippingType __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentShippingType;

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentCanMakePaymentResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentCanMakePaymentResultStatus
{
    PaymentCanMakePaymentResultStatus_Unknown = 0,
    PaymentCanMakePaymentResultStatus_Yes = 1,
    PaymentCanMakePaymentResultStatus_No = 2,
    PaymentCanMakePaymentResultStatus_NotAllowed = 3,
    PaymentCanMakePaymentResultStatus_UserNotSignedIn = 4,
    PaymentCanMakePaymentResultStatus_SpecifiedPaymentMethodIdsNotSupported = 5,
    PaymentCanMakePaymentResultStatus_NoQualifyingCardOnFile = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentOptionPresence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence
{
    PaymentOptionPresence_None = 0,
    PaymentOptionPresence_Optional = 1,
    PaymentOptionPresence_Required = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestChangeKind
{
    PaymentRequestChangeKind_ShippingOption = 0,
    PaymentRequestChangeKind_ShippingAddress = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestCompletionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus
{
    PaymentRequestCompletionStatus_Succeeded = 0,
    PaymentRequestCompletionStatus_Failed = 1,
    PaymentRequestCompletionStatus_Unknown = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentRequestStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestStatus
{
    PaymentRequestStatus_Succeeded = 0,
    PaymentRequestStatus_Failed = 1,
    PaymentRequestStatus_Canceled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.Payments.PaymentShippingType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentShippingType
{
    PaymentShippingType_Shipping = 0,
    PaymentShippingType_Delivery = 1,
    PaymentShippingType_Pickup = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Delegate Windows.ApplicationModel.Payments.PaymentRequestChangedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* paymentRequest,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* args);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandlerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_Invoke(This, paymentRequest, args) \
    ((This)->lpVtbl->Invoke(This, paymentRequest, args))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentAddress[] = L"Windows.ApplicationModel.Payments.IPaymentAddress";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Country)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Country)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AddressLines)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* put_AddressLines)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        __FIVectorView_1_HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Region)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Region)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_City)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_City)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DependentLocality)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DependentLocality)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PostalCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PostalCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SortingCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SortingCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LanguageCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_LanguageCode)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Organization)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Organization)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Recipient)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Recipient)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddressVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_Country(This, value) \
    ((This)->lpVtbl->get_Country(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_Country(This, value) \
    ((This)->lpVtbl->put_Country(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_AddressLines(This, value) \
    ((This)->lpVtbl->get_AddressLines(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_AddressLines(This, value) \
    ((This)->lpVtbl->put_AddressLines(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_Region(This, value) \
    ((This)->lpVtbl->get_Region(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_Region(This, value) \
    ((This)->lpVtbl->put_Region(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_City(This, value) \
    ((This)->lpVtbl->get_City(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_City(This, value) \
    ((This)->lpVtbl->put_City(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_DependentLocality(This, value) \
    ((This)->lpVtbl->get_DependentLocality(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_DependentLocality(This, value) \
    ((This)->lpVtbl->put_DependentLocality(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_PostalCode(This, value) \
    ((This)->lpVtbl->get_PostalCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_PostalCode(This, value) \
    ((This)->lpVtbl->put_PostalCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_SortingCode(This, value) \
    ((This)->lpVtbl->get_SortingCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_SortingCode(This, value) \
    ((This)->lpVtbl->put_SortingCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_LanguageCode(This, value) \
    ((This)->lpVtbl->get_LanguageCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_LanguageCode(This, value) \
    ((This)->lpVtbl->put_LanguageCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_Organization(This, value) \
    ((This)->lpVtbl->get_Organization(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_Organization(This, value) \
    ((This)->lpVtbl->put_Organization(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_Recipient(This, value) \
    ((This)->lpVtbl->get_Recipient(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_Recipient(This, value) \
    ((This)->lpVtbl->put_Recipient(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_PhoneNumber(This, value) \
    ((This)->lpVtbl->get_PhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_put_PhoneNumber(This, value) \
    ((This)->lpVtbl->put_PhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCanMakePaymentResult[] = L"Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentCanMakePaymentResultStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCanMakePaymentResultFactory[] = L"Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentCanMakePaymentResultStatus value,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_Create(This, value, result) \
    ((This)->lpVtbl->Create(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCanMakePaymentResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCurrencyAmount[] = L"Windows.ApplicationModel.Payments.IPaymentCurrencyAmount";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Currency)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Currency)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CurrencySystem)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CurrencySystem)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_get_Currency(This, value) \
    ((This)->lpVtbl->get_Currency(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_put_Currency(This, value) \
    ((This)->lpVtbl->put_Currency(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_get_CurrencySystem(This, value) \
    ((This)->lpVtbl->get_CurrencySystem(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_put_CurrencySystem(This, value) \
    ((This)->lpVtbl->put_CurrencySystem(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentCurrencyAmountFactory[] = L"Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        HSTRING value,
        HSTRING currency,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithCurrencySystem)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory* This,
        HSTRING value,
        HSTRING currency,
        HSTRING currencySystem,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_Create(This, value, currency, result) \
    ((This)->lpVtbl->Create(This, value, currency, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_CreateWithCurrencySystem(This, value, currency, currencySystem, result) \
    ((This)->lpVtbl->CreateWithCurrencySystem(This, value, currency, currencySystem, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmountFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetails[] = L"Windows.ApplicationModel.Payments.IPaymentDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Total)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** value);
    HRESULT (STDMETHODCALLTYPE* put_Total)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayItems)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem** value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayItems)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem* value);
    HRESULT (STDMETHODCALLTYPE* get_ShippingOptions)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption** value);
    HRESULT (STDMETHODCALLTYPE* put_ShippingOptions)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentShippingOption* value);
    HRESULT (STDMETHODCALLTYPE* get_Modifiers)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier** value);
    HRESULT (STDMETHODCALLTYPE* put_Modifiers)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentDetailsModifier* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_get_Total(This, value) \
    ((This)->lpVtbl->get_Total(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_put_Total(This, value) \
    ((This)->lpVtbl->put_Total(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_get_DisplayItems(This, value) \
    ((This)->lpVtbl->get_DisplayItems(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_put_DisplayItems(This, value) \
    ((This)->lpVtbl->put_DisplayItems(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_get_ShippingOptions(This, value) \
    ((This)->lpVtbl->get_ShippingOptions(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_put_ShippingOptions(This, value) \
    ((This)->lpVtbl->put_ShippingOptions(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_get_Modifiers(This, value) \
    ((This)->lpVtbl->get_Modifiers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_put_Modifiers(This, value) \
    ((This)->lpVtbl->put_Modifiers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsFactory[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* total,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithDisplayItems)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* total,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* displayItems,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_Create(This, total, result) \
    ((This)->lpVtbl->Create(This, total, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_CreateWithDisplayItems(This, total, displayItems, result) \
    ((This)->lpVtbl->CreateWithDisplayItems(This, total, displayItems, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsModifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsModifier[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsModifier";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_JsonData)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedMethodIds)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Total)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** value);
    HRESULT (STDMETHODCALLTYPE* get_AdditionalDisplayItems)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_get_JsonData(This, value) \
    ((This)->lpVtbl->get_JsonData(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_get_SupportedMethodIds(This, value) \
    ((This)->lpVtbl->get_SupportedMethodIds(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_get_Total(This, value) \
    ((This)->lpVtbl->get_Total(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_get_AdditionalDisplayItems(This, value) \
    ((This)->lpVtbl->get_AdditionalDisplayItems(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentDetailsModifierFactory[] = L"Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        __FIIterable_1_HSTRING* supportedMethodIds,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* total,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithAdditionalDisplayItems)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        __FIIterable_1_HSTRING* supportedMethodIds,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* total,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* additionalDisplayItems,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithAdditionalDisplayItemsAndJsonData)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory* This,
        __FIIterable_1_HSTRING* supportedMethodIds,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* total,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentItem* additionalDisplayItems,
        HSTRING jsonData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifier** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_Create(This, supportedMethodIds, total, result) \
    ((This)->lpVtbl->Create(This, supportedMethodIds, total, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_CreateWithAdditionalDisplayItems(This, supportedMethodIds, total, additionalDisplayItems, result) \
    ((This)->lpVtbl->CreateWithAdditionalDisplayItems(This, supportedMethodIds, total, additionalDisplayItems, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_CreateWithAdditionalDisplayItemsAndJsonData(This, supportedMethodIds, total, additionalDisplayItems, jsonData, result) \
    ((This)->lpVtbl->CreateWithAdditionalDisplayItemsAndJsonData(This, supportedMethodIds, total, additionalDisplayItems, jsonData, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetailsModifierFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentItem[] = L"Windows.ApplicationModel.Payments.IPaymentItem";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Label)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Amount)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount** value);
    HRESULT (STDMETHODCALLTYPE* put_Amount)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* value);
    HRESULT (STDMETHODCALLTYPE* get_Pending)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Pending)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_put_Label(This, value) \
    ((This)->lpVtbl->put_Label(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_get_Amount(This, value) \
    ((This)->lpVtbl->get_Amount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_put_Amount(This, value) \
    ((This)->lpVtbl->put_Amount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_get_Pending(This, value) \
    ((This)->lpVtbl->get_Pending(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_put_Pending(This, value) \
    ((This)->lpVtbl->put_Pending(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentItemFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentItemFactory[] = L"Windows.ApplicationModel.Payments.IPaymentItemFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* amount,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_Create(This, label, amount, result) \
    ((This)->lpVtbl->Create(This, label, amount, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentItemFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMediator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMediator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMediator[] = L"Windows.ApplicationModel.Payments.IPaymentMediator";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSupportedMethodIdsAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* SubmitPaymentRequestAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* paymentRequest,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult** result);
    HRESULT (STDMETHODCALLTYPE* SubmitPaymentRequestWithChangeHandlerAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* paymentRequest,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedHandler* changeHandler,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentRequestSubmitResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediatorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_GetSupportedMethodIdsAsync(This, result) \
    ((This)->lpVtbl->GetSupportedMethodIdsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_SubmitPaymentRequestAsync(This, paymentRequest, result) \
    ((This)->lpVtbl->SubmitPaymentRequestAsync(This, paymentRequest, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_SubmitPaymentRequestWithChangeHandlerAsync(This, paymentRequest, changeHandler, result) \
    ((This)->lpVtbl->SubmitPaymentRequestWithChangeHandlerAsync(This, paymentRequest, changeHandler, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMediator2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMediator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMediator2[] = L"Windows.ApplicationModel.Payments.IPaymentMediator2";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CanMakePaymentAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* paymentRequest,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPayments__CPaymentCanMakePaymentResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_CanMakePaymentAsync(This, paymentRequest, result) \
    ((This)->lpVtbl->CanMakePaymentAsync(This, paymentRequest, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMediator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMerchantInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMerchantInfo[] = L"Windows.ApplicationModel.Payments.IPaymentMerchantInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageFullName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_get_PackageFullName(This, value) \
    ((This)->lpVtbl->get_PackageFullName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMerchantInfoFactory[] = L"Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_Create(This, uri, result) \
    ((This)->lpVtbl->Create(This, uri, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMethodData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMethodData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMethodData[] = L"Windows.ApplicationModel.Payments.IPaymentMethodData";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedMethodIds)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_JsonData)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_get_SupportedMethodIds(This, value) \
    ((This)->lpVtbl->get_SupportedMethodIds(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_get_JsonData(This, value) \
    ((This)->lpVtbl->get_JsonData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentMethodDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentMethodData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentMethodDataFactory[] = L"Windows.ApplicationModel.Payments.IPaymentMethodDataFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        __FIIterable_1_HSTRING* supportedMethodIds,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithJsonData)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory* This,
        __FIIterable_1_HSTRING* supportedMethodIds,
        HSTRING jsonData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodData** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_Create(This, supportedMethodIds, result) \
    ((This)->lpVtbl->Create(This, supportedMethodIds, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_CreateWithJsonData(This, supportedMethodIds, jsonData, result) \
    ((This)->lpVtbl->CreateWithJsonData(This, supportedMethodIds, jsonData, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMethodDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentOptions[] = L"Windows.ApplicationModel.Payments.IPaymentOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestPayerEmail)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence* value);
    HRESULT (STDMETHODCALLTYPE* put_RequestPayerEmail)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence value);
    HRESULT (STDMETHODCALLTYPE* get_RequestPayerName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence* value);
    HRESULT (STDMETHODCALLTYPE* put_RequestPayerName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence value);
    HRESULT (STDMETHODCALLTYPE* get_RequestPayerPhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence* value);
    HRESULT (STDMETHODCALLTYPE* put_RequestPayerPhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentOptionPresence value);
    HRESULT (STDMETHODCALLTYPE* get_RequestShipping)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RequestShipping)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ShippingType)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentShippingType* value);
    HRESULT (STDMETHODCALLTYPE* put_ShippingType)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentShippingType value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_get_RequestPayerEmail(This, value) \
    ((This)->lpVtbl->get_RequestPayerEmail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_put_RequestPayerEmail(This, value) \
    ((This)->lpVtbl->put_RequestPayerEmail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_get_RequestPayerName(This, value) \
    ((This)->lpVtbl->get_RequestPayerName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_put_RequestPayerName(This, value) \
    ((This)->lpVtbl->put_RequestPayerName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_get_RequestPayerPhoneNumber(This, value) \
    ((This)->lpVtbl->get_RequestPayerPhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_put_RequestPayerPhoneNumber(This, value) \
    ((This)->lpVtbl->put_RequestPayerPhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_get_RequestShipping(This, value) \
    ((This)->lpVtbl->get_RequestShipping(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_put_RequestShipping(This, value) \
    ((This)->lpVtbl->put_RequestShipping(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_get_ShippingType(This, value) \
    ((This)->lpVtbl->get_ShippingType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_put_ShippingType(This, value) \
    ((This)->lpVtbl->put_ShippingType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequest[] = L"Windows.ApplicationModel.Payments.IPaymentRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MerchantInfo)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_MethodData)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        __FIVectorView_1_Windows__CApplicationModel__CPayments__CPaymentMethodData** value);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_get_MerchantInfo(This, value) \
    ((This)->lpVtbl->get_MerchantInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_get_MethodData(This, value) \
    ((This)->lpVtbl->get_MethodData(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequest2[] = L"Windows.ApplicationModel.Payments.IPaymentRequest2";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedArgs[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeKind)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestChangeKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ShippingAddress)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedShippingOption)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** value);
    HRESULT (STDMETHODCALLTYPE* Acknowledge)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* changeResult);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_get_ChangeKind(This, value) \
    ((This)->lpVtbl->get_ChangeKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_get_ShippingAddress(This, value) \
    ((This)->lpVtbl->get_ShippingAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_get_SelectedShippingOption(This, value) \
    ((This)->lpVtbl->get_SelectedShippingOption(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_Acknowledge(This, changeResult) \
    ((This)->lpVtbl->Acknowledge(This, changeResult))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedResult[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeAcceptedByMerchant)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ChangeAcceptedByMerchant)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Message)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_UpdatedPaymentDetails)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails** value);
    HRESULT (STDMETHODCALLTYPE* put_UpdatedPaymentDetails)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_get_ChangeAcceptedByMerchant(This, value) \
    ((This)->lpVtbl->get_ChangeAcceptedByMerchant(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_put_ChangeAcceptedByMerchant(This, value) \
    ((This)->lpVtbl->put_ChangeAcceptedByMerchant(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_put_Message(This, value) \
    ((This)->lpVtbl->put_Message(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_get_UpdatedPaymentDetails(This, value) \
    ((This)->lpVtbl->get_UpdatedPaymentDetails(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_put_UpdatedPaymentDetails(This, value) \
    ((This)->lpVtbl->put_UpdatedPaymentDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestChangedResultFactory[] = L"Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        boolean changeAcceptedByMerchant,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithPaymentDetails)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory* This,
        boolean changeAcceptedByMerchant,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* updatedPaymentDetails,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_Create(This, changeAcceptedByMerchant, result) \
    ((This)->lpVtbl->Create(This, changeAcceptedByMerchant, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_CreateWithPaymentDetails(This, changeAcceptedByMerchant, updatedPaymentDetails, result) \
    ((This)->lpVtbl->CreateWithPaymentDetails(This, changeAcceptedByMerchant, updatedPaymentDetails, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestChangedResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestFactory[] = L"Windows.ApplicationModel.Payments.IPaymentRequestFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* details,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithMerchantInfo)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* details,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* merchantInfo,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithMerchantInfoAndOptions)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* details,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* merchantInfo,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* options,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_Create(This, details, methodData, result) \
    ((This)->lpVtbl->Create(This, details, methodData, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_CreateWithMerchantInfo(This, details, methodData, merchantInfo, result) \
    ((This)->lpVtbl->CreateWithMerchantInfo(This, details, methodData, merchantInfo, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_CreateWithMerchantInfoAndOptions(This, details, methodData, merchantInfo, options, result) \
    ((This)->lpVtbl->CreateWithMerchantInfoAndOptions(This, details, methodData, merchantInfo, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestFactory2[] = L"Windows.ApplicationModel.Payments.IPaymentRequestFactory2";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithMerchantInfoOptionsAndId)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentDetails* details,
        __FIIterable_1_Windows__CApplicationModel__CPayments__CPaymentMethodData* methodData,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentMerchantInfo* merchantInfo,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentOptions* options,
        HSTRING id,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequest** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_CreateWithMerchantInfoOptionsAndId(This, details, methodData, merchantInfo, options, id, result) \
    ((This)->lpVtbl->CreateWithMerchantInfoOptionsAndId(This, details, methodData, merchantInfo, options, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentRequestSubmitResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentRequestSubmitResult[] = L"Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_get_Response(This, value) \
    ((This)->lpVtbl->get_Response(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentRequestSubmitResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentResponse
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentResponse[] = L"Windows.ApplicationModel.Payments.IPaymentResponse";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PaymentToken)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken** value);
    HRESULT (STDMETHODCALLTYPE* get_ShippingOption)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** value);
    HRESULT (STDMETHODCALLTYPE* get_ShippingAddress)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentAddress** value);
    HRESULT (STDMETHODCALLTYPE* get_PayerEmail)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PayerName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PayerPhoneNumber)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* CompleteAsync)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse* This,
        enum __x_ABI_CWindows_CApplicationModel_CPayments_CPaymentRequestCompletionStatus status,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponseVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_PaymentToken(This, value) \
    ((This)->lpVtbl->get_PaymentToken(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_ShippingOption(This, value) \
    ((This)->lpVtbl->get_ShippingOption(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_ShippingAddress(This, value) \
    ((This)->lpVtbl->get_ShippingAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_PayerEmail(This, value) \
    ((This)->lpVtbl->get_PayerEmail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_PayerName(This, value) \
    ((This)->lpVtbl->get_PayerName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_get_PayerPhoneNumber(This, value) \
    ((This)->lpVtbl->get_PayerPhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_CompleteAsync(This, status, result) \
    ((This)->lpVtbl->CompleteAsync(This, status, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentResponse_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentShippingOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentShippingOption[] = L"Windows.ApplicationModel.Payments.IPaymentShippingOption";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Label)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Amount)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount** value);
    HRESULT (STDMETHODCALLTYPE* put_Amount)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsSelected)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSelected)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_put_Label(This, value) \
    ((This)->lpVtbl->put_Label(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_get_Amount(This, value) \
    ((This)->lpVtbl->get_Amount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_put_Amount(This, value) \
    ((This)->lpVtbl->put_Amount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_get_IsSelected(This, value) \
    ((This)->lpVtbl->get_IsSelected(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_put_IsSelected(This, value) \
    ((This)->lpVtbl->put_IsSelected(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentShippingOptionFactory[] = L"Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* amount,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithSelected)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* amount,
        boolean selected,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithSelectedAndTag)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory* This,
        HSTRING label,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentCurrencyAmount* amount,
        boolean selected,
        HSTRING tag,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOption** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_Create(This, label, amount, result) \
    ((This)->lpVtbl->Create(This, label, amount, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_CreateWithSelected(This, label, amount, selected, result) \
    ((This)->lpVtbl->CreateWithSelected(This, label, amount, selected, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_CreateWithSelectedAndTag(This, label, amount, selected, tag, result) \
    ((This)->lpVtbl->CreateWithSelectedAndTag(This, label, amount, selected, tag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentShippingOptionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentToken
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentToken[] = L"Windows.ApplicationModel.Payments.IPaymentToken";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PaymentMethodId)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JsonDetails)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_get_PaymentMethodId(This, value) \
    ((This)->lpVtbl->get_PaymentMethodId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_get_JsonDetails(This, value) \
    ((This)->lpVtbl->get_JsonDetails(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Payments.IPaymentTokenFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Payments.PaymentToken
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Payments_IPaymentTokenFactory[] = L"Windows.ApplicationModel.Payments.IPaymentTokenFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        HSTRING paymentMethodId,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithJsonDetails)(__x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory* This,
        HSTRING paymentMethodId,
        HSTRING jsonDetails,
        __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentToken** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_Create(This, paymentMethodId, result) \
    ((This)->lpVtbl->Create(This, paymentMethodId, result))

#define __x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_CreateWithJsonDetails(This, paymentMethodId, jsonDetails, result) \
    ((This)->lpVtbl->CreateWithJsonDetails(This, paymentMethodId, jsonDetails, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CPayments_CIPaymentTokenFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentAddress_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentAddress[] = L"Windows.ApplicationModel.Payments.PaymentAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResultFactory interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentCanMakePaymentResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentCanMakePaymentResult[] = L"Windows.ApplicationModel.Payments.PaymentCanMakePaymentResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentCurrencyAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentCurrencyAmountFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentCurrencyAmount ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCurrencyAmount_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentCurrencyAmount_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentCurrencyAmount[] = L"Windows.ApplicationModel.Payments.PaymentCurrencyAmount";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentDetailsFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentDetails[] = L"Windows.ApplicationModel.Payments.PaymentDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentDetailsModifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentDetailsModifierFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentDetailsModifier ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetailsModifier_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentDetailsModifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentDetailsModifier[] = L"Windows.ApplicationModel.Payments.PaymentDetailsModifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentItemFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentItem ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentItem[] = L"Windows.ApplicationModel.Payments.PaymentItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMediator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMediator ** Default Interface **
 *    Windows.ApplicationModel.Payments.IPaymentMediator2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMediator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMediator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMediator[] = L"Windows.ApplicationModel.Payments.PaymentMediator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMerchantInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentMerchantInfoFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMerchantInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMerchantInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMerchantInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMerchantInfo[] = L"Windows.ApplicationModel.Payments.PaymentMerchantInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentMethodData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentMethodDataFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentMethodData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMethodData_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentMethodData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentMethodData[] = L"Windows.ApplicationModel.Payments.PaymentMethodData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentOptions[] = L"Windows.ApplicationModel.Payments.PaymentOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestFactory2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequest ** Default Interface **
 *    Windows.ApplicationModel.Payments.IPaymentRequest2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequest[] = L"Windows.ApplicationModel.Payments.PaymentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestChangedArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestChangedArgs[] = L"Windows.ApplicationModel.Payments.PaymentRequestChangedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestChangedResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentRequestChangedResultFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestChangedResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestChangedResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestChangedResult[] = L"Windows.ApplicationModel.Payments.PaymentRequestChangedResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentRequestSubmitResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentRequestSubmitResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentRequestSubmitResult[] = L"Windows.ApplicationModel.Payments.PaymentRequestSubmitResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentResponse ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentResponse_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentResponse_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentResponse[] = L"Windows.ApplicationModel.Payments.PaymentResponse";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentShippingOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentShippingOptionFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentShippingOption ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentShippingOption_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentShippingOption_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentShippingOption[] = L"Windows.ApplicationModel.Payments.PaymentShippingOption";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.Payments.PaymentToken
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Payments.IPaymentTokenFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Payments.IPaymentToken ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentToken_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Payments_PaymentToken_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Payments_PaymentToken[] = L"Windows.ApplicationModel.Payments.PaymentToken";
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
#endif // __windows2Eapplicationmodel2Epayments_p_h__

#endif // __windows2Eapplicationmodel2Epayments_h__
