
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
#ifndef __windows2Eapplicationmodel2Econversationalagent_h__
#define __windows2Eapplicationmodel2Econversationalagent_h__
#ifndef __windows2Eapplicationmodel2Econversationalagent_p_h__
#define __windows2Eapplicationmodel2Econversationalagent_p_h__


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
#include "Windows.Media.Audio.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IActivationSignalDetectionConfiguration;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IActivationSignalDetectionConfiguration2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2 ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IActivationSignalDetectionConfigurationCreationResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfigurationCreationResult

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IActivationSignalDetector;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IActivationSignalDetector2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2 ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentDetectorManager;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentDetectorManager

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentDetectorManager2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2 ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentDetectorManager2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentDetectorManagerStatics;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentDetectorManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSession;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSession2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2 ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSessionInterruptedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSessionInterruptedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSessionStatics;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSessionStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSignal;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSignal

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSignal2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2 ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSignal2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSignalDetectedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSignalDetectedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IConversationalAgentSystemStateChangedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSystemStateChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IDetectionConfigurationAvailabilityChangedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs ABI::Windows::ApplicationModel::ConversationalAgent::IDetectionConfigurationAvailabilityChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IDetectionConfigurationAvailabilityInfo;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo ABI::Windows::ApplicationModel::ConversationalAgent::IDetectionConfigurationAvailabilityInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                interface IDetectionConfigurationAvailabilityInfo2;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2 ABI::Windows::ApplicationModel::ConversationalAgent::IDetectionConfigurationAvailabilityInfo2

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__

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



#ifndef DEF___FIAsyncOperation_1_IInspectable_USE
#define DEF___FIAsyncOperation_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abf53c57-ee50-5342-b52a-26e3b8cc024f"))
IAsyncOperation<IInspectable*> : IAsyncOperation_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<IInspectable*> __FIAsyncOperation_1_IInspectable_t;
#define __FIAsyncOperation_1_IInspectable ABI::Windows::Foundation::__FIAsyncOperation_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_IInspectable_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE
#define DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3f08262e-a2e1-5134-9297-e9211f481a2d"))
IAsyncOperationCompletedHandler<IInspectable*> : IAsyncOperationCompletedHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<IInspectable*> __FIAsyncOperationCompletedHandler_1_IInspectable_t;
#define __FIAsyncOperationCompletedHandler_1_IInspectable ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_IInspectable_USE */



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ActivationSignalDetectionConfiguration;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("754c4ad1-97e2-5443-8558-5d9d5305d01b"))
IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f2c357e2-b348-5451-82a3-f00048c3a217"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ActivationSignalDetectionConfigurationCreationResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("edd52757-74d4-5837-941f-c37c5d4b4f53"))
IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfigurationCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("67d722c6-e9e6-56ef-bd63-96fc1a6d0292"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfigurationCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectionConfigurationRemovalResult : int ActivationSignalDetectionConfigurationRemovalResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("312794cb-0e43-5cbf-a148-58a9d7a65c14"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df53d38d-347f-587d-b115-be893a8d7c49"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectionConfigurationSetModelDataResult : int ActivationSignalDetectionConfigurationSetModelDataResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3738e73c-e4cd-55ad-b760-07af8b4a29d3"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c3982503-9ae9-582e-9874-33ea36649a3c"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectionConfigurationStateChangeResult : int ActivationSignalDetectionConfigurationStateChangeResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("be03cc1f-0a8a-5878-a84e-af4646df3cc5"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4aa91bfb-908e-52ba-a23d-06f464b5b54a"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ActivationSignalDetector;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3566eb2e-4f9c-5261-998d-90296eff00c5"))
IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("550e15d0-52f2-5369-bed9-646c9589140b"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentActivationResult : int ConversationalAgentActivationResult;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d177b8f3-d984-5c78-a57f-f096e291b374"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("41948ce0-5d83-5e12-a8ba-114c4a3a62e3"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentSession;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51831e09-6f91-59b0-820d-60b97775c575"))
IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("077a96a1-7932-5db3-a503-34a30571f3f2"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentSessionUpdateResponse : int ConversationalAgentSessionUpdateResponse;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b85887b-8070-5ef9-aac8-92515257061e"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6f43477a-01c5-59e2-96e2-1d9a00409159"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum DetectionConfigurationTrainingStatus : int DetectionConfigurationTrainingStatus;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7631a007-93a6-554e-bcdd-3baee7c8c2c6"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus> __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c131b87-097f-5965-84d5-964c52b1787e"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000


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



#ifndef DEF___FIIterator_1_UINT32_USE
#define DEF___FIIterator_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f06a2739-9443-5ef0-b284-dc5aff3e7d10"))
IIterator<UINT32> : IIterator_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT32> __FIIterator_1_UINT32_t;
#define __FIIterator_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT32_USE */



#ifndef DEF___FIIterable_1_UINT32_USE
#define DEF___FIIterable_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("421d4b91-b13b-5f37-ae54-b5249bd80539"))
IIterable<UINT32> : IIterable_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT32> __FIIterable_1_UINT32_t;
#define __FIIterable_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT32_USE */



#ifndef DEF___FIVectorView_1_UINT32_USE
#define DEF___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5ce1a07-8d33-5007-ba64-7d2508ccf85c"))
IVectorView<UINT32> : IVectorView_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT32> __FIVectorView_1_UINT32_t;
#define __FIVectorView_1_UINT32 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT32_USE */



#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_UINT32_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("52c56f3c-713a-5162-9e62-362ce7ed53be"))
IAsyncOperation<__FIVectorView_1_UINT32*> : IAsyncOperation_impl<__FIVectorView_1_UINT32*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<UInt32>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_UINT32*> __FIAsyncOperation_1___FIVectorView_1_UINT32_t;
#define __FIAsyncOperation_1___FIVectorView_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("55772f29-da64-5c87-871c-074337a84573"))
IAsyncOperationCompletedHandler<__FIVectorView_1_UINT32*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_UINT32*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<UInt32>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_UINT32*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23e55a59-d7b5-5a4f-a51d-45d156776e3f"))
IIterator<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cb03ac8c-3a88-5c35-9872-00a466e49a12"))
IIterable<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2515803e-4d85-5df7-a88a-88388730d659"))
IVectorView<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec02c465-2697-5e66-b867-1411dd7d57a7"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3678f40b-3b3b-539b-8b7c-6f3ab257fb75"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bc1c2e1c-bdc3-50b8-96ab-2e12531f6bc0"))
IIterator<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e1f74a8f-38eb-58be-b770-78af5a4a6b2e"))
IIterable<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6e514658-da3d-5201-9eb2-b2ef1c8dd1ba"))
IVectorView<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetector*> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ecc1a135-a8a5-51b9-8e1a-6340d9f56cae"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0ec801b3-4434-5d72-9a12-926ed91738a9"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentVoiceActivationPrerequisiteKind : int ConversationalAgentVoiceActivationPrerequisiteKind;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("89b54546-b694-5277-866b-7f2fc740c640"))
IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> : IIterator_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e9bb4c0-65ab-5049-9336-918b6d359071"))
IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> : IIterable_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("82216c11-1e7a-592c-9a3d-15d63e6fdd19"))
IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> : IVectorView_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentVoiceActivationPrerequisiteKind> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c192099d-c28a-550a-8c2b-e31c8fabcb4b"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80bbbe11-f1c5-5a42-b20b-005a57f88011"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92b02cd3-aa6e-573d-bc03-8d2309cba3eb"))
IAsyncOperation<__FIVector_1_HSTRING*> : IAsyncOperation_impl<__FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVector_1_HSTRING*> __FIAsyncOperation_1___FIVector_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVector_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVector_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVector_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fae4b396-97c8-5cc3-bf88-ea3098edf6b2"))
IAsyncOperationCompletedHandler<__FIVector_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVector_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVector`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVector_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioDeviceInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioDeviceInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode ABI::Windows::Media::Audio::IAudioDeviceInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d009b9cb-e9c1-5d8d-9575-c33ac26ce44a"))
IAsyncOperation<ABI::Windows::Media::Audio::AudioDeviceInputNode*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioDeviceInputNode*, ABI::Windows::Media::Audio::IAudioDeviceInputNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.AudioDeviceInputNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::AudioDeviceInputNode*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("19b1586d-db7d-55e8-9729-2256bd9984d4"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::AudioDeviceInputNode*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioDeviceInputNode*, ABI::Windows::Media::Audio::IAudioDeviceInputNode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.AudioDeviceInputNode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::AudioDeviceInputNode*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8fe0732-556d-5841-b7ee-b3450fb52666"))
IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d0bd0125-9049-57a3-bd66-e2525d98c814"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IInputStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IInputStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IInputStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectionTrainingDataFormat : int ActivationSignalDetectionTrainingDataFormat;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1f6ef295-4d31-5874-a22d-29875ee8165c"))
IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> : IIterator_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("88b67e2c-ec3f-520c-ae65-bfe63272fadd"))
IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> : IIterable_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectorPowerState : int ActivationSignalDetectorPowerState;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f5daac9c-92cf-5945-9c5b-1c0b8eac8143"))
IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> : IIterator_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6860d48-cec5-5a00-8653-4c924a991575"))
IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> : IIterable_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum SignalDetectorResourceKind : int SignalDetectorResourceKind;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a27176ec-ab9a-5a55-8e50-ad4dab5b5d68"))
IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> : IIterator_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t;
#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("263e7b6e-5338-5871-9fdb-a27d9bc02e92"))
IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> : IIterable_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t;
#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("679189c6-73c4-5295-8b04-6ce05cab4d17"))
IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> : IVectorView_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c303fd7f-d073-54c3-8411-c118226323ac"))
IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> : IVectorView_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorPowerState> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a847ac9b-a9ae-55c5-811d-920122e361d6"))
IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> : IVectorView_impl<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::ConversationalAgent::SignalDetectorResourceKind> __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t;
#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class DetectionConfigurationAvailabilityChangedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("491146ea-2bc6-5f03-a317-097aae055198"))
ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationAvailabilityChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationAvailabilityChangedEventArgs*, ABI::Windows::ApplicationModel::ConversationalAgent::IDetectionConfigurationAvailabilityChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration, Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfiguration*, ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationAvailabilityChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentSessionInterruptedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e1c093f2-c2f4-58c6-9fd1-3beb13b18ec8"))
ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionInterruptedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionInterruptedEventArgs*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSessionInterruptedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession, Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionInterruptedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentSignalDetectedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d4b78ffb-98b2-5004-9cb4-24dd755734fb"))
ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSignalDetectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSignalDetectedEventArgs*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSignalDetectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession, Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSignalDetectedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentSystemStateChangedEventArgs;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ac7da0c7-d0d3-5bac-bbc9-52ad49131f1f"))
ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSystemStateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSystemStateChangedEventArgs*, ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSystemStateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession, Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSession*, ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSystemStateChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraph;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraph;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph ABI::Windows::Media::Audio::IAudioGraph

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectionConfigurationCreationStatus : int ActivationSignalDetectionConfigurationCreationStatus;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ActivationSignalDetectorKind : int ActivationSignalDetectorKind;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentActivationKind : int ConversationalAgentActivationKind;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentState : int ConversationalAgentState;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum ConversationalAgentSystemStateChangeType : int ConversationalAgentSystemStateChangeType;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                typedef enum DetectionConfigurationAvailabilityChangeKind : int DetectionConfigurationAvailabilityChangeKind;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentDetectorManager;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class ConversationalAgentSignal;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                class DetectionConfigurationAvailabilityInfo;
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectionConfigurationCreationStatus : int
                {
                    ActivationSignalDetectionConfigurationCreationStatus_Success = 0,
                    ActivationSignalDetectionConfigurationCreationStatus_SignalIdNotAvailable = 1,
                    ActivationSignalDetectionConfigurationCreationStatus_ModelIdNotSupported = 2,
                    ActivationSignalDetectionConfigurationCreationStatus_InvalidSignalId = 3,
                    ActivationSignalDetectionConfigurationCreationStatus_InvalidModelId = 4,
                    ActivationSignalDetectionConfigurationCreationStatus_InvalidDisplayName = 5,
                    ActivationSignalDetectionConfigurationCreationStatus_ConfigurationAlreadyExists = 6,
                    ActivationSignalDetectionConfigurationCreationStatus_CreationNotSupported = 7,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectionConfigurationRemovalResult : int
                {
                    ActivationSignalDetectionConfigurationRemovalResult_Success = 0,
                    ActivationSignalDetectionConfigurationRemovalResult_NotFound = 1,
                    ActivationSignalDetectionConfigurationRemovalResult_CurrentlyEnabled = 2,
                    ActivationSignalDetectionConfigurationRemovalResult_RemovalNotSupported = 3,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectionConfigurationSetModelDataResult : int
                {
                    ActivationSignalDetectionConfigurationSetModelDataResult_Success = 0,
                    ActivationSignalDetectionConfigurationSetModelDataResult_EmptyModelData = 1,
                    ActivationSignalDetectionConfigurationSetModelDataResult_UnsupportedFormat = 2,
                    ActivationSignalDetectionConfigurationSetModelDataResult_ConfigurationCurrentlyEnabled = 3,
                    ActivationSignalDetectionConfigurationSetModelDataResult_InvalidData = 4,
                    ActivationSignalDetectionConfigurationSetModelDataResult_SetModelDataNotSupported = 5,
                    ActivationSignalDetectionConfigurationSetModelDataResult_ConfigurationNotFound = 6,
                    ActivationSignalDetectionConfigurationSetModelDataResult_UnknownError = 7,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectionConfigurationStateChangeResult : int
                {
                    ActivationSignalDetectionConfigurationStateChangeResult_Success = 0,
                    ActivationSignalDetectionConfigurationStateChangeResult_NoModelData = 1,
                    ActivationSignalDetectionConfigurationStateChangeResult_ConfigurationNotFound = 2,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectionTrainingDataFormat : int
                {
                    ActivationSignalDetectionTrainingDataFormat_Voice8kHz8BitMono = 0,
                    ActivationSignalDetectionTrainingDataFormat_Voice8kHz16BitMono = 1,
                    ActivationSignalDetectionTrainingDataFormat_Voice16kHz8BitMono = 2,
                    ActivationSignalDetectionTrainingDataFormat_Voice16kHz16BitMono = 3,
                    ActivationSignalDetectionTrainingDataFormat_VoiceOEMDefined = 4,
                    ActivationSignalDetectionTrainingDataFormat_Audio44kHz8BitMono = 5,
                    ActivationSignalDetectionTrainingDataFormat_Audio44kHz16BitMono = 6,
                    ActivationSignalDetectionTrainingDataFormat_Audio48kHz8BitMono = 7,
                    ActivationSignalDetectionTrainingDataFormat_Audio48kHz16BitMono = 8,
                    ActivationSignalDetectionTrainingDataFormat_AudioOEMDefined = 9,
                    ActivationSignalDetectionTrainingDataFormat_OtherOEMDefined = 10,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectorKind : int
                {
                    ActivationSignalDetectorKind_AudioPattern = 0,
                    ActivationSignalDetectorKind_AudioImpulse = 1,
                    ActivationSignalDetectorKind_HardwareEvent = 2,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ActivationSignalDetectorPowerState : int
                {
                    ActivationSignalDetectorPowerState_HighPower = 0,
                    ActivationSignalDetectorPowerState_ConnectedLowPower = 1,
                    ActivationSignalDetectorPowerState_DisconnectedLowPower = 2,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentActivationKind : int
                {
                    ConversationalAgentActivationKind_VoiceActivationPreview = 0,
                    ConversationalAgentActivationKind_Foreground = 1,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentActivationResult : int
                {
                    ConversationalAgentActivationResult_Success = 0,
                    ConversationalAgentActivationResult_AgentInactive = 1,
                    ConversationalAgentActivationResult_ScreenNotAvailable = 2,
                    ConversationalAgentActivationResult_AgentInterrupted = 3,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentSessionUpdateResponse : int
                {
                    ConversationalAgentSessionUpdateResponse_Success = 0,
                    ConversationalAgentSessionUpdateResponse_Failed = 1,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentState : int
                {
                    ConversationalAgentState_Inactive = 0,
                    ConversationalAgentState_Detecting = 1,
                    ConversationalAgentState_Listening = 2,
                    ConversationalAgentState_Working = 3,
                    ConversationalAgentState_Speaking = 4,
                    ConversationalAgentState_ListeningAndSpeaking = 5,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentSystemStateChangeType : int
                {
                    ConversationalAgentSystemStateChangeType_UserAuthentication = 0,
                    ConversationalAgentSystemStateChangeType_ScreenAvailability = 1,
                    ConversationalAgentSystemStateChangeType_IndicatorLightAvailability = 2,
                    ConversationalAgentSystemStateChangeType_VoiceActivationAvailability = 3,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum ConversationalAgentVoiceActivationPrerequisiteKind : int
                {
                    ConversationalAgentVoiceActivationPrerequisiteKind_MicrophonePermission = 0,
                    ConversationalAgentVoiceActivationPrerequisiteKind_KnownAgents = 1,
                    ConversationalAgentVoiceActivationPrerequisiteKind_AgentAllowed = 2,
                    ConversationalAgentVoiceActivationPrerequisiteKind_AppCapability = 3,
                    ConversationalAgentVoiceActivationPrerequisiteKind_BackgroundTaskRegistration = 4,
                    ConversationalAgentVoiceActivationPrerequisiteKind_PolicyPermission = 5,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum DetectionConfigurationAvailabilityChangeKind : int
                {
                    DetectionConfigurationAvailabilityChangeKind_SystemResourceAccess = 0,
                    DetectionConfigurationAvailabilityChangeKind_Permission = 1,
                    DetectionConfigurationAvailabilityChangeKind_LockScreenPermission = 2,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum DetectionConfigurationTrainingStatus : int
                {
                    DetectionConfigurationTrainingStatus_Success = 0,
                    DetectionConfigurationTrainingStatus_FormatNotSupported = 1,
                    DetectionConfigurationTrainingStatus_VoiceTooQuiet = 2,
                    DetectionConfigurationTrainingStatus_VoiceTooLoud = 3,
                    DetectionConfigurationTrainingStatus_VoiceTooFast = 4,
                    DetectionConfigurationTrainingStatus_VoiceTooSlow = 5,
                    DetectionConfigurationTrainingStatus_VoiceQualityProblem = 6,
                    DetectionConfigurationTrainingStatus_TrainingSystemInternalError = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    DetectionConfigurationTrainingStatus_TrainingTimedOut = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    DetectionConfigurationTrainingStatus_ConfigurationNotFound = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                enum SignalDetectorResourceKind : int
                {
                    SignalDetectorResourceKind_ParallelModelSupport = 0,
                    SignalDetectorResourceKind_ParallelModelSupportForAgent = 1,
                    SignalDetectorResourceKind_ParallelSignalSupport = 2,
                    SignalDetectorResourceKind_ParallelSignalSupportForAgent = 3,
                    SignalDetectorResourceKind_DisplayOffSupport = 4,
                    SignalDetectorResourceKind_PluggedInPower = 5,
                    SignalDetectorResourceKind_Detector = 6,
                    SignalDetectorResourceKind_SupportedSleepState = 7,
                    SignalDetectorResourceKind_SupportedBatterySaverState = 8,
                    SignalDetectorResourceKind_ScreenAvailability = 9,
                    SignalDetectorResourceKind_InputHardware = 10,
                    SignalDetectorResourceKind_AcousticEchoCancellation = 11,
                    SignalDetectorResourceKind_ModelIdSupport = 12,
                    SignalDetectorResourceKind_DataChannel = 13,
                };
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfiguration[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("40d8be16-5217-581c-9ab2-ce9b2f2e8e00")
                IActivationSignalDetectionConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SignalId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEnabledAsync(
                        boolean value,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AvailabilityInfo(
                        ABI::Windows::ApplicationModel::ConversationalAgent::IDetectionConfigurationAvailabilityInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AvailabilityChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AvailabilityChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetModelData(
                        HSTRING dataType,
                        ABI::Windows::Storage::Streams::IInputStream* data
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetModelDataAsync(
                        HSTRING dataType,
                        ABI::Windows::Storage::Streams::IInputStream* data,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetModelDataType(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetModelDataTypeAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetModelData(
                        ABI::Windows::Storage::Streams::IInputStream** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetModelDataAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearModelData(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearModelDataAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrainingStepsCompleted(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrainingStepsRemaining(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrainingDataFormat(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApplyTrainingData(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat trainingDataFormat,
                        ABI::Windows::Storage::Streams::IInputStream* trainingData,
                        ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationTrainingStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApplyTrainingDataAsync(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionTrainingDataFormat trainingDataFormat,
                        ABI::Windows::Storage::Streams::IInputStream* trainingData,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearTrainingData(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearTrainingDataAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationSignalDetectionConfiguration = __uuidof(IActivationSignalDetectionConfiguration);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfiguration2[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("71d9b022-562c-57ce-a78b-8b4ff0145bab")
                IActivationSignalDetectionConfiguration2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetModelDataWithResult(
                        HSTRING dataType,
                        ABI::Windows::Storage::Streams::IInputStream* data,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationSetModelDataResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetModelDataWithResultAsync(
                        HSTRING dataType,
                        ABI::Windows::Storage::Streams::IInputStream* data,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEnabledWithResultAsync(
                        boolean value,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEnabledWithResult(
                        boolean value,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationStateChangeResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrainingStepCompletionMaxAllowedTime(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationSignalDetectionConfiguration2 = __uuidof(IActivationSignalDetectionConfiguration2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfigurationCreationResult[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0")
                IActivationSignalDetectionConfigurationCreationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationSignalDetectionConfigurationCreationResult = __uuidof(IActivationSignalDetectionConfigurationCreationResult);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetector[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff")
                IActivationSignalDetector : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanCreateConfigurations(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedModelDataTypes(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedTrainingDataFormats(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedPowerStates(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedModelIdsForSignalId(
                        HSTRING signalId,
                        __FIVectorView_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedModelIdsForSignalIdAsync(
                        HSTRING signalId,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateConfiguration(
                        HSTRING signalId,
                        HSTRING modelId,
                        HSTRING displayName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateConfigurationAsync(
                        HSTRING signalId,
                        HSTRING modelId,
                        HSTRING displayName,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConfigurations(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConfigurationsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConfiguration(
                        HSTRING signalId,
                        HSTRING modelId,
                        ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConfigurationAsync(
                        HSTRING signalId,
                        HSTRING modelId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveConfiguration(
                        HSTRING signalId,
                        HSTRING modelId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveConfigurationAsync(
                        HSTRING signalId,
                        HSTRING modelId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationSignalDetector = __uuidof(IActivationSignalDetector);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetector2[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("c7e2490a-baa5-59d2-85d1-ba42f7cf78c9")
                IActivationSignalDetector2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAvailableModelIdsForSignalIdAsync(
                        HSTRING signalId,
                        __FIAsyncOperation_1___FIVector_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAvailableModelIdsForSignalId(
                        HSTRING signalId,
                        __FIVector_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateConfigurationWithResultAsync(
                        HSTRING signalId,
                        HSTRING modelId,
                        HSTRING displayName,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateConfigurationWithResult(
                        HSTRING signalId,
                        HSTRING modelId,
                        HSTRING displayName,
                        ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetectionConfigurationCreationResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveConfigurationWithResultAsync(
                        HSTRING signalId,
                        HSTRING modelId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveConfigurationWithResult(
                        HSTRING signalId,
                        HSTRING modelId,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectionConfigurationRemovalResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetectorId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActivationSignalDetector2 = __uuidof(IActivationSignalDetector2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManager[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("de94fbb0-597a-5df8-8cfb-9dbb583ba3ff")
                IConversationalAgentDetectorManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAllActivationSignalDetectors(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAllActivationSignalDetectorsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActivationSignalDetectors(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorKind kind,
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActivationSignalDetectorsAsync(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorKind kind,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentDetectorManager = __uuidof(IConversationalAgentDetectorManager);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManager2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("84610f31-d7f3-52fe-9311-c9eb4e3eb30a")
                IConversationalAgentDetectorManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetActivationSignalDetectorFromId(
                        HSTRING detectorId,
                        ABI::Windows::ApplicationModel::ConversationalAgent::IActivationSignalDetector** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActivationSignalDetectorFromIdAsync(
                        HSTRING detectorId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentDetectorManager2 = __uuidof(IConversationalAgentDetectorManager2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManagerStatics[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("36a8d283-fa0e-5693-8489-0fb2f0ab40d3")
                IConversationalAgentDetectorManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Default(
                        ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentDetectorManager** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentDetectorManagerStatics = __uuidof(IConversationalAgentDetectorManagerStatics);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSession[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("daaae09a-b7ba-57e5-ad13-df520f9b6fa7")
                IConversationalAgentSession : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_SessionInterrupted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SessionInterrupted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SignalDetected(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SignalDetected(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SystemStateChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SystemStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AgentState(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Signal(
                        ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSignal** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIndicatorLightAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsScreenAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsUserAuthenticated(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVoiceActivationAvailable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInterruptible(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsInterrupted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestInterruptibleAsync(
                        boolean interruptible,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestInterruptible(
                        boolean interruptible,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAgentStateChangeAsync(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentState state,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAgentStateChange(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentState state,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestForegroundActivationAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestForegroundActivation(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSessionUpdateResponse* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioClientAsync(
                        __FIAsyncOperation_1_IInspectable** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioClient(
                        IInspectable** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioDeviceInputNodeAsync(
                        ABI::Windows::Media::Audio::IAudioGraph* graph,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioDeviceInputNode(
                        ABI::Windows::Media::Audio::IAudioGraph* graph,
                        ABI::Windows::Media::Audio::IAudioDeviceInputNode** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioCaptureDeviceIdAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioCaptureDeviceId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioRenderDeviceIdAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioRenderDeviceId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSignalModelIdAsync(
                        __FIAsyncOperation_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSignalModelId(
                        UINT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSignalModelIdAsync(
                        UINT32 signalModelId,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSignalModelId(
                        UINT32 signalModelId,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedSignalModelIdsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_UINT32** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedSignalModelIds(
                        __FIVectorView_1_UINT32** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSession = __uuidof(IConversationalAgentSession);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSession2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("a7a9fbf9-ac78-57ff-9596-acc7a1c9a607")
                IConversationalAgentSession2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestActivationAsync(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationKind activationKind,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestActivation(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationKind activationKind,
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentActivationResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSupportLockScreenActivationAsync(
                        boolean lockScreenActivationSupported,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSupportLockScreenActivation(
                        boolean lockScreenActivationSupported
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMissingPrerequisites(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMissingPrerequisitesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSession2 = __uuidof(IConversationalAgentSession2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSessionInterruptedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("9766591f-f63d-5d3e-9bf2-bd0760552686")
                IConversationalAgentSessionInterruptedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSessionInterruptedEventArgs = __uuidof(IConversationalAgentSessionInterruptedEventArgs);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSessionStatics[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("a005166e-e954-576e-be04-11b8ed10f37b")
                IConversationalAgentSessionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentSessionAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentSessionSync(
                        ABI::Windows::ApplicationModel::ConversationalAgent::IConversationalAgentSession** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSessionStatics = __uuidof(IConversationalAgentSessionStatics);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignal[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("20ed25f7-b120-51f2-8603-265d6a47f232")
                IConversationalAgentSignal : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSignalVerificationRequired(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSignalVerificationRequired(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignalId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignalName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalContext(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignalContext(
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalStart(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignalStart(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalEnd(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SignalEnd(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSignal = __uuidof(IConversationalAgentSignal);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignal2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("d0cc7ba9-9a7b-5c34-880e-b6146c904ecb")
                IConversationalAgentSignal2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DetectorId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetectorKind(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ActivationSignalDetectorKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSignal2 = __uuidof(IConversationalAgentSignal2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignalDetectedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("4d57eb8f-f88a-599b-91d3-d604876708bc")
                IConversationalAgentSignalDetectedEventArgs : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSignalDetectedEventArgs = __uuidof(IConversationalAgentSignalDetectedEventArgs);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSystemStateChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("1c2c6e3e-2785-59a7-8e71-38adeef79928")
                IConversationalAgentSystemStateChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SystemStateChangeType(
                        ABI::Windows::ApplicationModel::ConversationalAgent::ConversationalAgentSystemStateChangeType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IConversationalAgentSystemStateChangedEventArgs = __uuidof(IConversationalAgentSystemStateChangedEventArgs);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("5129c9fb-4be8-5f14-af2b-88d62b1b4462")
                IDetectionConfigurationAvailabilityChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::ConversationalAgent::DetectionConfigurationAvailabilityChangeKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDetectionConfigurationAvailabilityChangedEventArgs = __uuidof(IDetectionConfigurationAvailabilityChangedEventArgs);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityInfo[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("b5affeb0-40f0-5398-b838-91979c2c6208")
                IDetectionConfigurationAvailabilityInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasSystemResourceAccess(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasPermission(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasLockScreenPermission(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDetectionConfigurationAvailabilityInfo = __uuidof(IDetectionConfigurationAvailabilityInfo);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityInfo2[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ConversationalAgent {
                MIDL_INTERFACE("30e06433-38b3-5c4b-84c3-62b6e685b2ff")
                IDetectionConfigurationAvailabilityInfo2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UnavailableSystemResources(
                        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDetectionConfigurationAvailabilityInfo2 = __uuidof(IDetectionConfigurationAvailabilityInfo2);
            } /* ConversationalAgent */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo[] = L"Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2 __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2;

#endif // ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_IInspectable __FIAsyncOperationCompletedHandler_1_IInspectable;

#if !defined(____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_IInspectable __FIAsyncOperation_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_IInspectable;

typedef struct __FIAsyncOperation_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1_IInspectable* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_IInspectable* This,
        __FIAsyncOperationCompletedHandler_1_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIAsyncOperation_1_IInspectableVtbl;

interface __FIAsyncOperation_1_IInspectable
{
    CONST_VTBL struct __FIAsyncOperation_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_IInspectable_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_IInspectable_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_IInspectable_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_IInspectable __FIAsyncOperationCompletedHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_IInspectable;

typedef struct __FIAsyncOperationCompletedHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_IInspectable* This,
        __FIAsyncOperation_1_IInspectable* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_IInspectableVtbl;

interface __FIAsyncOperationCompletedHandler_1_IInspectable
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_IInspectable_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationRemovalResult __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationRemovalResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationRemovalResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationSetModelDataResult __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationSetModelDataResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationSetModelDataResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationStateChangeResult __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationStateChangeResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationStateChangeResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationResult __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationTrainingStatus __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationTrainingStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationTrainingStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

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

#if !defined(____FIIterator_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT32 __FIIterator_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT32;

typedef struct __FIIterator_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT32Vtbl;

interface __FIIterator_1_UINT32
{
    CONST_VTBL struct __FIIterator_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT32 __FIIterable_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT32;

typedef struct __FIIterable_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT32* This,
        __FIIterator_1_UINT32** result);

    END_INTERFACE
} __FIIterable_1_UINT32Vtbl;

interface __FIIterable_1_UINT32
{
    CONST_VTBL struct __FIIterable_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT32 __FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT32;

typedef struct __FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT32Vtbl;

interface __FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32 __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32;

#if !defined(____FIAsyncOperation_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_UINT32 __FIAsyncOperation_1___FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_UINT32;

typedef struct __FIAsyncOperation_1___FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_UINT32* This,
        __FIVectorView_1_UINT32** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_UINT32Vtbl;

interface __FIAsyncOperation_1___FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32 __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32* This,
        __FIAsyncOperation_1___FIVectorView_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVector_1_HSTRING __FIAsyncOperation_1___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVector_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVector_1_HSTRING* This,
        __FIVector_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVector_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING* This,
        __FIAsyncOperation_1___FIVector_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVector_1_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioDeviceInputNode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIInputStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

typedef struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

typedef struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        __FIIterator_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKindVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* sender,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* sender,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* sender,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* sender,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationCreationStatus __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationCreationStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationKind __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState;

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSystemStateChangeType __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSystemStateChangeType;

typedef enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationAvailabilityChangeKind __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationAvailabilityChangeKind;

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationCreationStatus
{
    ActivationSignalDetectionConfigurationCreationStatus_Success = 0,
    ActivationSignalDetectionConfigurationCreationStatus_SignalIdNotAvailable = 1,
    ActivationSignalDetectionConfigurationCreationStatus_ModelIdNotSupported = 2,
    ActivationSignalDetectionConfigurationCreationStatus_InvalidSignalId = 3,
    ActivationSignalDetectionConfigurationCreationStatus_InvalidModelId = 4,
    ActivationSignalDetectionConfigurationCreationStatus_InvalidDisplayName = 5,
    ActivationSignalDetectionConfigurationCreationStatus_ConfigurationAlreadyExists = 6,
    ActivationSignalDetectionConfigurationCreationStatus_CreationNotSupported = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationRemovalResult
{
    ActivationSignalDetectionConfigurationRemovalResult_Success = 0,
    ActivationSignalDetectionConfigurationRemovalResult_NotFound = 1,
    ActivationSignalDetectionConfigurationRemovalResult_CurrentlyEnabled = 2,
    ActivationSignalDetectionConfigurationRemovalResult_RemovalNotSupported = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationSetModelDataResult
{
    ActivationSignalDetectionConfigurationSetModelDataResult_Success = 0,
    ActivationSignalDetectionConfigurationSetModelDataResult_EmptyModelData = 1,
    ActivationSignalDetectionConfigurationSetModelDataResult_UnsupportedFormat = 2,
    ActivationSignalDetectionConfigurationSetModelDataResult_ConfigurationCurrentlyEnabled = 3,
    ActivationSignalDetectionConfigurationSetModelDataResult_InvalidData = 4,
    ActivationSignalDetectionConfigurationSetModelDataResult_SetModelDataNotSupported = 5,
    ActivationSignalDetectionConfigurationSetModelDataResult_ConfigurationNotFound = 6,
    ActivationSignalDetectionConfigurationSetModelDataResult_UnknownError = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationStateChangeResult
{
    ActivationSignalDetectionConfigurationStateChangeResult_Success = 0,
    ActivationSignalDetectionConfigurationStateChangeResult_NoModelData = 1,
    ActivationSignalDetectionConfigurationStateChangeResult_ConfigurationNotFound = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat
{
    ActivationSignalDetectionTrainingDataFormat_Voice8kHz8BitMono = 0,
    ActivationSignalDetectionTrainingDataFormat_Voice8kHz16BitMono = 1,
    ActivationSignalDetectionTrainingDataFormat_Voice16kHz8BitMono = 2,
    ActivationSignalDetectionTrainingDataFormat_Voice16kHz16BitMono = 3,
    ActivationSignalDetectionTrainingDataFormat_VoiceOEMDefined = 4,
    ActivationSignalDetectionTrainingDataFormat_Audio44kHz8BitMono = 5,
    ActivationSignalDetectionTrainingDataFormat_Audio44kHz16BitMono = 6,
    ActivationSignalDetectionTrainingDataFormat_Audio48kHz8BitMono = 7,
    ActivationSignalDetectionTrainingDataFormat_Audio48kHz16BitMono = 8,
    ActivationSignalDetectionTrainingDataFormat_AudioOEMDefined = 9,
    ActivationSignalDetectionTrainingDataFormat_OtherOEMDefined = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind
{
    ActivationSignalDetectorKind_AudioPattern = 0,
    ActivationSignalDetectorKind_AudioImpulse = 1,
    ActivationSignalDetectorKind_HardwareEvent = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorPowerState
{
    ActivationSignalDetectorPowerState_HighPower = 0,
    ActivationSignalDetectorPowerState_ConnectedLowPower = 1,
    ActivationSignalDetectorPowerState_DisconnectedLowPower = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationKind
{
    ConversationalAgentActivationKind_VoiceActivationPreview = 0,
    ConversationalAgentActivationKind_Foreground = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationResult
{
    ConversationalAgentActivationResult_Success = 0,
    ConversationalAgentActivationResult_AgentInactive = 1,
    ConversationalAgentActivationResult_ScreenNotAvailable = 2,
    ConversationalAgentActivationResult_AgentInterrupted = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse
{
    ConversationalAgentSessionUpdateResponse_Success = 0,
    ConversationalAgentSessionUpdateResponse_Failed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState
{
    ConversationalAgentState_Inactive = 0,
    ConversationalAgentState_Detecting = 1,
    ConversationalAgentState_Listening = 2,
    ConversationalAgentState_Working = 3,
    ConversationalAgentState_Speaking = 4,
    ConversationalAgentState_ListeningAndSpeaking = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSystemStateChangeType
{
    ConversationalAgentSystemStateChangeType_UserAuthentication = 0,
    ConversationalAgentSystemStateChangeType_ScreenAvailability = 1,
    ConversationalAgentSystemStateChangeType_IndicatorLightAvailability = 2,
    ConversationalAgentSystemStateChangeType_VoiceActivationAvailability = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentVoiceActivationPrerequisiteKind
{
    ConversationalAgentVoiceActivationPrerequisiteKind_MicrophonePermission = 0,
    ConversationalAgentVoiceActivationPrerequisiteKind_KnownAgents = 1,
    ConversationalAgentVoiceActivationPrerequisiteKind_AgentAllowed = 2,
    ConversationalAgentVoiceActivationPrerequisiteKind_AppCapability = 3,
    ConversationalAgentVoiceActivationPrerequisiteKind_BackgroundTaskRegistration = 4,
    ConversationalAgentVoiceActivationPrerequisiteKind_PolicyPermission = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationAvailabilityChangeKind
{
    DetectionConfigurationAvailabilityChangeKind_SystemResourceAccess = 0,
    DetectionConfigurationAvailabilityChangeKind_Permission = 1,
    DetectionConfigurationAvailabilityChangeKind_LockScreenPermission = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationTrainingStatus
{
    DetectionConfigurationTrainingStatus_Success = 0,
    DetectionConfigurationTrainingStatus_FormatNotSupported = 1,
    DetectionConfigurationTrainingStatus_VoiceTooQuiet = 2,
    DetectionConfigurationTrainingStatus_VoiceTooLoud = 3,
    DetectionConfigurationTrainingStatus_VoiceTooFast = 4,
    DetectionConfigurationTrainingStatus_VoiceTooSlow = 5,
    DetectionConfigurationTrainingStatus_VoiceQualityProblem = 6,
    DetectionConfigurationTrainingStatus_TrainingSystemInternalError = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DetectionConfigurationTrainingStatus_TrainingTimedOut = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    DetectionConfigurationTrainingStatus_ConfigurationNotFound = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CSignalDetectorResourceKind
{
    SignalDetectorResourceKind_ParallelModelSupport = 0,
    SignalDetectorResourceKind_ParallelModelSupportForAgent = 1,
    SignalDetectorResourceKind_ParallelSignalSupport = 2,
    SignalDetectorResourceKind_ParallelSignalSupportForAgent = 3,
    SignalDetectorResourceKind_DisplayOffSupport = 4,
    SignalDetectorResourceKind_PluggedInPower = 5,
    SignalDetectorResourceKind_Detector = 6,
    SignalDetectorResourceKind_SupportedSleepState = 7,
    SignalDetectorResourceKind_SupportedBatterySaverState = 8,
    SignalDetectorResourceKind_ScreenAvailability = 9,
    SignalDetectorResourceKind_InputHardware = 10,
    SignalDetectorResourceKind_AcousticEchoCancellation = 11,
    SignalDetectorResourceKind_ModelIdSupport = 12,
    SignalDetectorResourceKind_DataChannel = 13,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfiguration[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SignalId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetEnabled)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetEnabledAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        boolean value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_AvailabilityInfo)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo** value);
    HRESULT (STDMETHODCALLTYPE* add_AvailabilityChanged)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationAvailabilityChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AvailabilityChanged)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetModelData)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING dataType,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data);
    HRESULT (STDMETHODCALLTYPE* SetModelDataAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING dataType,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetModelDataType)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetModelDataTypeAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetModelData)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** result);
    HRESULT (STDMETHODCALLTYPE* GetModelDataAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIInputStream** operation);
    HRESULT (STDMETHODCALLTYPE* ClearModelData)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* ClearModelDataAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_TrainingStepsCompleted)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TrainingStepsRemaining)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TrainingDataFormat)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat* value);
    HRESULT (STDMETHODCALLTYPE* ApplyTrainingData)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat trainingDataFormat,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* trainingData,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationTrainingStatus* result);
    HRESULT (STDMETHODCALLTYPE* ApplyTrainingDataAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionTrainingDataFormat trainingDataFormat,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* trainingData,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CDetectionConfigurationTrainingStatus** operation);
    HRESULT (STDMETHODCALLTYPE* ClearTrainingData)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* ClearTrainingDataAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_SignalId(This, value) \
    ((This)->lpVtbl->get_SignalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_ModelId(This, value) \
    ((This)->lpVtbl->get_ModelId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_SetEnabled(This, value) \
    ((This)->lpVtbl->SetEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_SetEnabledAsync(This, value, operation) \
    ((This)->lpVtbl->SetEnabledAsync(This, value, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_AvailabilityInfo(This, value) \
    ((This)->lpVtbl->get_AvailabilityInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_add_AvailabilityChanged(This, handler, token) \
    ((This)->lpVtbl->add_AvailabilityChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_remove_AvailabilityChanged(This, token) \
    ((This)->lpVtbl->remove_AvailabilityChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_SetModelData(This, dataType, data) \
    ((This)->lpVtbl->SetModelData(This, dataType, data))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_SetModelDataAsync(This, dataType, data, operation) \
    ((This)->lpVtbl->SetModelDataAsync(This, dataType, data, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetModelDataType(This, result) \
    ((This)->lpVtbl->GetModelDataType(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetModelDataTypeAsync(This, operation) \
    ((This)->lpVtbl->GetModelDataTypeAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetModelData(This, result) \
    ((This)->lpVtbl->GetModelData(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_GetModelDataAsync(This, operation) \
    ((This)->lpVtbl->GetModelDataAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ClearModelData(This) \
    ((This)->lpVtbl->ClearModelData(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ClearModelDataAsync(This, operation) \
    ((This)->lpVtbl->ClearModelDataAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_TrainingStepsCompleted(This, value) \
    ((This)->lpVtbl->get_TrainingStepsCompleted(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_TrainingStepsRemaining(This, value) \
    ((This)->lpVtbl->get_TrainingStepsRemaining(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_get_TrainingDataFormat(This, value) \
    ((This)->lpVtbl->get_TrainingDataFormat(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ApplyTrainingData(This, trainingDataFormat, trainingData, result) \
    ((This)->lpVtbl->ApplyTrainingData(This, trainingDataFormat, trainingData, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ApplyTrainingDataAsync(This, trainingDataFormat, trainingData, operation) \
    ((This)->lpVtbl->ApplyTrainingDataAsync(This, trainingDataFormat, trainingData, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ClearTrainingData(This) \
    ((This)->lpVtbl->ClearTrainingData(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_ClearTrainingDataAsync(This, operation) \
    ((This)->lpVtbl->ClearTrainingDataAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfiguration2[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetModelDataWithResult)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        HSTRING dataType,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationSetModelDataResult* result);
    HRESULT (STDMETHODCALLTYPE* SetModelDataWithResultAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        HSTRING dataType,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* data,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationSetModelDataResult** operation);
    HRESULT (STDMETHODCALLTYPE* SetEnabledWithResultAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        boolean value,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationStateChangeResult** operation);
    HRESULT (STDMETHODCALLTYPE* SetEnabledWithResult)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        boolean value,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationStateChangeResult* result);
    HRESULT (STDMETHODCALLTYPE* get_TrainingStepCompletionMaxAllowedTime)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_SetModelDataWithResult(This, dataType, data, result) \
    ((This)->lpVtbl->SetModelDataWithResult(This, dataType, data, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_SetModelDataWithResultAsync(This, dataType, data, operation) \
    ((This)->lpVtbl->SetModelDataWithResultAsync(This, dataType, data, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_SetEnabledWithResultAsync(This, value, operation) \
    ((This)->lpVtbl->SetEnabledWithResultAsync(This, value, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_SetEnabledWithResult(This, value, result) \
    ((This)->lpVtbl->SetEnabledWithResult(This, value, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_get_TrainingStepCompletionMaxAllowedTime(This, value) \
    ((This)->lpVtbl->get_TrainingStepCompletionMaxAllowedTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetectionConfigurationCreationResult[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_get_Configuration(This, value) \
    ((This)->lpVtbl->get_Configuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetector[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_CanCreateConfigurations)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedModelDataTypes)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedTrainingDataFormats)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionTrainingDataFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedPowerStates)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectorPowerState** value);
    HRESULT (STDMETHODCALLTYPE* GetSupportedModelIdsForSignalId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetSupportedModelIdsForSignalIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* CreateConfiguration)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId,
        HSTRING displayName);
    HRESULT (STDMETHODCALLTYPE* CreateConfigurationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId,
        HSTRING displayName,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* GetConfigurations)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetConfigurationsAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** operation);
    HRESULT (STDMETHODCALLTYPE* GetConfiguration)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* GetConfigurationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfiguration** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveConfiguration)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId);
    HRESULT (STDMETHODCALLTYPE* RemoveConfigurationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector* This,
        HSTRING signalId,
        HSTRING modelId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_CanCreateConfigurations(This, value) \
    ((This)->lpVtbl->get_CanCreateConfigurations(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_SupportedModelDataTypes(This, value) \
    ((This)->lpVtbl->get_SupportedModelDataTypes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_SupportedTrainingDataFormats(This, value) \
    ((This)->lpVtbl->get_SupportedTrainingDataFormats(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_get_SupportedPowerStates(This, value) \
    ((This)->lpVtbl->get_SupportedPowerStates(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetSupportedModelIdsForSignalId(This, signalId, result) \
    ((This)->lpVtbl->GetSupportedModelIdsForSignalId(This, signalId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetSupportedModelIdsForSignalIdAsync(This, signalId, operation) \
    ((This)->lpVtbl->GetSupportedModelIdsForSignalIdAsync(This, signalId, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_CreateConfiguration(This, signalId, modelId, displayName) \
    ((This)->lpVtbl->CreateConfiguration(This, signalId, modelId, displayName))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_CreateConfigurationAsync(This, signalId, modelId, displayName, operation) \
    ((This)->lpVtbl->CreateConfigurationAsync(This, signalId, modelId, displayName, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetConfigurations(This, result) \
    ((This)->lpVtbl->GetConfigurations(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetConfigurationsAsync(This, operation) \
    ((This)->lpVtbl->GetConfigurationsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetConfiguration(This, signalId, modelId, result) \
    ((This)->lpVtbl->GetConfiguration(This, signalId, modelId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_GetConfigurationAsync(This, signalId, modelId, operation) \
    ((This)->lpVtbl->GetConfigurationAsync(This, signalId, modelId, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_RemoveConfiguration(This, signalId, modelId) \
    ((This)->lpVtbl->RemoveConfiguration(This, signalId, modelId))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_RemoveConfigurationAsync(This, signalId, modelId, operation) \
    ((This)->lpVtbl->RemoveConfigurationAsync(This, signalId, modelId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IActivationSignalDetector2[] = L"Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAvailableModelIdsForSignalIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        __FIAsyncOperation_1___FIVector_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetAvailableModelIdsForSignalId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        __FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* CreateConfigurationWithResultAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        HSTRING modelId,
        HSTRING displayName,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationCreationResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateConfigurationWithResult)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        HSTRING modelId,
        HSTRING displayName,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetectionConfigurationCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* RemoveConfigurationWithResultAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        HSTRING modelId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetectionConfigurationRemovalResult** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveConfigurationWithResult)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING signalId,
        HSTRING modelId,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectionConfigurationRemovalResult* result);
    HRESULT (STDMETHODCALLTYPE* get_DetectorId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_GetAvailableModelIdsForSignalIdAsync(This, signalId, operation) \
    ((This)->lpVtbl->GetAvailableModelIdsForSignalIdAsync(This, signalId, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_GetAvailableModelIdsForSignalId(This, signalId, result) \
    ((This)->lpVtbl->GetAvailableModelIdsForSignalId(This, signalId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_CreateConfigurationWithResultAsync(This, signalId, modelId, displayName, operation) \
    ((This)->lpVtbl->CreateConfigurationWithResultAsync(This, signalId, modelId, displayName, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_CreateConfigurationWithResult(This, signalId, modelId, displayName, result) \
    ((This)->lpVtbl->CreateConfigurationWithResult(This, signalId, modelId, displayName, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_RemoveConfigurationWithResultAsync(This, signalId, modelId, operation) \
    ((This)->lpVtbl->RemoveConfigurationWithResultAsync(This, signalId, modelId, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_RemoveConfigurationWithResult(This, signalId, modelId, result) \
    ((This)->lpVtbl->RemoveConfigurationWithResult(This, signalId, modelId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_get_DetectorId(This, value) \
    ((This)->lpVtbl->get_DetectorId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManager[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAllActivationSignalDetectors)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetAllActivationSignalDetectorsAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation);
    HRESULT (STDMETHODCALLTYPE* GetActivationSignalDetectors)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind kind,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetActivationSignalDetectorsAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind kind,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetAllActivationSignalDetectors(This, result) \
    ((This)->lpVtbl->GetAllActivationSignalDetectors(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetAllActivationSignalDetectorsAsync(This, operation) \
    ((This)->lpVtbl->GetAllActivationSignalDetectorsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetActivationSignalDetectors(This, kind, result) \
    ((This)->lpVtbl->GetActivationSignalDetectors(This, kind, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_GetActivationSignalDetectorsAsync(This, kind, operation) \
    ((This)->lpVtbl->GetActivationSignalDetectorsAsync(This, kind, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManager2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetActivationSignalDetectorFromId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        HSTRING detectorId,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIActivationSignalDetector** result);
    HRESULT (STDMETHODCALLTYPE* GetActivationSignalDetectorFromIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2* This,
        HSTRING detectorId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CActivationSignalDetector** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_GetActivationSignalDetectorFromId(This, detectorId, result) \
    ((This)->lpVtbl->GetActivationSignalDetectorFromId(This, detectorId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_GetActivationSignalDetectorFromIdAsync(This, detectorId, operation) \
    ((This)->lpVtbl->GetActivationSignalDetectorFromIdAsync(This, detectorId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentDetectorManagerStatics[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Default)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_get_Default(This, value) \
    ((This)->lpVtbl->get_Default(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentDetectorManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSession[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SessionInterrupted)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionInterruptedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SessionInterrupted)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SignalDetected)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSignalDetectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SignalDetected)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SystemStateChanged)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSystemStateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SystemStateChanged)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_AgentState)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState* value);
    HRESULT (STDMETHODCALLTYPE* get_Signal)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal** value);
    HRESULT (STDMETHODCALLTYPE* get_IsIndicatorLightAvailable)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsScreenAvailable)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsUserAuthenticated)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVoiceActivationAvailable)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInterruptible)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsInterrupted)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* RequestInterruptibleAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean interruptible,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation);
    HRESULT (STDMETHODCALLTYPE* RequestInterruptible)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        boolean interruptible,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse* result);
    HRESULT (STDMETHODCALLTYPE* RequestAgentStateChangeAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState state,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation);
    HRESULT (STDMETHODCALLTYPE* RequestAgentStateChange)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentState state,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse* result);
    HRESULT (STDMETHODCALLTYPE* RequestForegroundActivationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSessionUpdateResponse** operation);
    HRESULT (STDMETHODCALLTYPE* RequestForegroundActivation)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSessionUpdateResponse* result);
    HRESULT (STDMETHODCALLTYPE* GetAudioClientAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1_IInspectable** operation);
    HRESULT (STDMETHODCALLTYPE* GetAudioClient)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* CreateAudioDeviceInputNodeAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* graph,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioDeviceInputNode** operation);
    HRESULT (STDMETHODCALLTYPE* CreateAudioDeviceInputNode)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* graph,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode** result);
    HRESULT (STDMETHODCALLTYPE* GetAudioCaptureDeviceIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetAudioCaptureDeviceId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetAudioRenderDeviceIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* GetAudioRenderDeviceId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetSignalModelIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* GetSignalModelId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* SetSignalModelIdAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        UINT32 signalModelId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* SetSignalModelId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        UINT32 signalModelId,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetSupportedSignalModelIdsAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIAsyncOperation_1___FIVectorView_1_UINT32** operation);
    HRESULT (STDMETHODCALLTYPE* GetSupportedSignalModelIds)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession* This,
        __FIVectorView_1_UINT32** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_add_SessionInterrupted(This, handler, token) \
    ((This)->lpVtbl->add_SessionInterrupted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_remove_SessionInterrupted(This, token) \
    ((This)->lpVtbl->remove_SessionInterrupted(This, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_add_SignalDetected(This, handler, token) \
    ((This)->lpVtbl->add_SignalDetected(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_remove_SignalDetected(This, token) \
    ((This)->lpVtbl->remove_SignalDetected(This, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_add_SystemStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_SystemStateChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_remove_SystemStateChanged(This, token) \
    ((This)->lpVtbl->remove_SystemStateChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_AgentState(This, value) \
    ((This)->lpVtbl->get_AgentState(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_Signal(This, value) \
    ((This)->lpVtbl->get_Signal(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsIndicatorLightAvailable(This, value) \
    ((This)->lpVtbl->get_IsIndicatorLightAvailable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsScreenAvailable(This, value) \
    ((This)->lpVtbl->get_IsScreenAvailable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsUserAuthenticated(This, value) \
    ((This)->lpVtbl->get_IsUserAuthenticated(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsVoiceActivationAvailable(This, value) \
    ((This)->lpVtbl->get_IsVoiceActivationAvailable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsInterruptible(This, value) \
    ((This)->lpVtbl->get_IsInterruptible(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_get_IsInterrupted(This, value) \
    ((This)->lpVtbl->get_IsInterrupted(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestInterruptibleAsync(This, interruptible, operation) \
    ((This)->lpVtbl->RequestInterruptibleAsync(This, interruptible, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestInterruptible(This, interruptible, result) \
    ((This)->lpVtbl->RequestInterruptible(This, interruptible, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestAgentStateChangeAsync(This, state, operation) \
    ((This)->lpVtbl->RequestAgentStateChangeAsync(This, state, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestAgentStateChange(This, state, result) \
    ((This)->lpVtbl->RequestAgentStateChange(This, state, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestForegroundActivationAsync(This, operation) \
    ((This)->lpVtbl->RequestForegroundActivationAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_RequestForegroundActivation(This, result) \
    ((This)->lpVtbl->RequestForegroundActivation(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioClientAsync(This, operation) \
    ((This)->lpVtbl->GetAudioClientAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioClient(This, result) \
    ((This)->lpVtbl->GetAudioClient(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_CreateAudioDeviceInputNodeAsync(This, graph, operation) \
    ((This)->lpVtbl->CreateAudioDeviceInputNodeAsync(This, graph, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_CreateAudioDeviceInputNode(This, graph, result) \
    ((This)->lpVtbl->CreateAudioDeviceInputNode(This, graph, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioCaptureDeviceIdAsync(This, operation) \
    ((This)->lpVtbl->GetAudioCaptureDeviceIdAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioCaptureDeviceId(This, result) \
    ((This)->lpVtbl->GetAudioCaptureDeviceId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioRenderDeviceIdAsync(This, operation) \
    ((This)->lpVtbl->GetAudioRenderDeviceIdAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetAudioRenderDeviceId(This, result) \
    ((This)->lpVtbl->GetAudioRenderDeviceId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetSignalModelIdAsync(This, operation) \
    ((This)->lpVtbl->GetSignalModelIdAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetSignalModelId(This, result) \
    ((This)->lpVtbl->GetSignalModelId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_SetSignalModelIdAsync(This, signalModelId, operation) \
    ((This)->lpVtbl->SetSignalModelIdAsync(This, signalModelId, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_SetSignalModelId(This, signalModelId, result) \
    ((This)->lpVtbl->SetSignalModelId(This, signalModelId, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetSupportedSignalModelIdsAsync(This, operation) \
    ((This)->lpVtbl->GetSupportedSignalModelIdsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_GetSupportedSignalModelIds(This, result) \
    ((This)->lpVtbl->GetSupportedSignalModelIds(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSession2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestActivationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationKind activationKind,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentActivationResult** operation);
    HRESULT (STDMETHODCALLTYPE* RequestActivation)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationKind activationKind,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentActivationResult* result);
    HRESULT (STDMETHODCALLTYPE* SetSupportLockScreenActivationAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        boolean lockScreenActivationSupported,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* SetSupportLockScreenActivation)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        boolean lockScreenActivationSupported);
    HRESULT (STDMETHODCALLTYPE* GetMissingPrerequisites)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** result);
    HRESULT (STDMETHODCALLTYPE* GetMissingPrerequisitesAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentVoiceActivationPrerequisiteKind** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_RequestActivationAsync(This, activationKind, operation) \
    ((This)->lpVtbl->RequestActivationAsync(This, activationKind, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_RequestActivation(This, activationKind, result) \
    ((This)->lpVtbl->RequestActivation(This, activationKind, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_SetSupportLockScreenActivationAsync(This, lockScreenActivationSupported, operation) \
    ((This)->lpVtbl->SetSupportLockScreenActivationAsync(This, lockScreenActivationSupported, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_SetSupportLockScreenActivation(This, lockScreenActivationSupported) \
    ((This)->lpVtbl->SetSupportLockScreenActivation(This, lockScreenActivationSupported))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_GetMissingPrerequisites(This, result) \
    ((This)->lpVtbl->GetMissingPrerequisites(This, result))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_GetMissingPrerequisitesAsync(This, operation) \
    ((This)->lpVtbl->GetMissingPrerequisitesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSessionInterruptedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionInterruptedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSessionStatics[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentSessionAsync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CConversationalAgent__CConversationalAgentSession** operation);
    HRESULT (STDMETHODCALLTYPE* GetCurrentSessionSync)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics* This,
        __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSession** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_GetCurrentSessionAsync(This, operation) \
    ((This)->lpVtbl->GetCurrentSessionAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_GetCurrentSessionSync(This, result) \
    ((This)->lpVtbl->GetCurrentSessionSync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSessionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignal[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSignalVerificationRequired)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSignalVerificationRequired)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SignalId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SignalId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SignalName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SignalName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SignalContext)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* put_SignalContext)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* get_SignalStart)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_SignalStart)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_SignalEnd)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_SignalEnd)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_IsSignalVerificationRequired(This, value) \
    ((This)->lpVtbl->get_IsSignalVerificationRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_IsSignalVerificationRequired(This, value) \
    ((This)->lpVtbl->put_IsSignalVerificationRequired(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_SignalId(This, value) \
    ((This)->lpVtbl->get_SignalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_SignalId(This, value) \
    ((This)->lpVtbl->put_SignalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_SignalName(This, value) \
    ((This)->lpVtbl->get_SignalName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_SignalName(This, value) \
    ((This)->lpVtbl->put_SignalName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_SignalContext(This, value) \
    ((This)->lpVtbl->get_SignalContext(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_SignalContext(This, value) \
    ((This)->lpVtbl->put_SignalContext(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_SignalStart(This, value) \
    ((This)->lpVtbl->get_SignalStart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_SignalStart(This, value) \
    ((This)->lpVtbl->put_SignalStart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_get_SignalEnd(This, value) \
    ((This)->lpVtbl->get_SignalEnd(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_put_SignalEnd(This, value) \
    ((This)->lpVtbl->put_SignalEnd(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignal2[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DetectorId)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DetectorKind)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CActivationSignalDetectorKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_get_DetectorId(This, value) \
    ((This)->lpVtbl->get_DetectorId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_get_DetectorKind(This, value) \
    ((This)->lpVtbl->get_DetectorKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignal2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSignalDetectedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSignalDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IConversationalAgentSystemStateChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SystemStateChangeType)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CConversationalAgentSystemStateChangeType* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_get_SystemStateChangeType(This, value) \
    ((This)->lpVtbl->get_SystemStateChangeType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIConversationalAgentSystemStateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CDetectionConfigurationAvailabilityChangeKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityInfo[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasSystemResourceAccess)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasPermission)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HasLockScreenPermission)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_get_HasSystemResourceAccess(This, value) \
    ((This)->lpVtbl->get_HasSystemResourceAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_get_HasPermission(This, value) \
    ((This)->lpVtbl->get_HasPermission(This, value))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_get_HasLockScreenPermission(This, value) \
    ((This)->lpVtbl->get_HasLockScreenPermission(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ConversationalAgent_IDetectionConfigurationAvailabilityInfo2[] = L"Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2";
typedef struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnavailableSystemResources)(__x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2* This,
        __FIVectorView_1_Windows__CApplicationModel__CConversationalAgent__CSignalDetectorResourceKind** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_get_UnavailableSystemResources(This, value) \
    ((This)->lpVtbl->get_UnavailableSystemResources(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CConversationalAgent_CIDetectionConfigurationAvailabilityInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfiguration[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetectionConfigurationCreationResult[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ActivationSignalDetector[] = L"Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentDetectorManager[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSession[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSessionInterruptedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignal[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSignalDetectedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_ConversationalAgentSystemStateChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityChangedEventArgs[] = L"Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo ** Default Interface **
 *    Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ConversationalAgent_DetectionConfigurationAvailabilityInfo[] = L"Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Econversationalagent_p_h__

#endif // __windows2Eapplicationmodel2Econversationalagent_h__
