
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
#ifndef __windows2Eapplicationmodel2Eextendedexecution2Eforeground_h__
#define __windows2Eapplicationmodel2Eextendedexecution2Eforeground_h__
#ifndef __windows2Eapplicationmodel2Eextendedexecution2Eforeground_p_h__
#define __windows2Eapplicationmodel2Eextendedexecution2Eforeground_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    interface IExtendedExecutionForegroundRevokedEventArgs;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundRevokedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    interface IExtendedExecutionForegroundSession;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundSession

#endif // ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    typedef enum ExtendedExecutionForegroundResult : int ExtendedExecutionForegroundResult;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b18ea00f-8c20-5ac2-9246-3ef405271f1a"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult> __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07e1dc01-18ba-596a-b745-79f9cde44ccc"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundResult> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    class ExtendedExecutionForegroundRevokedEventArgs;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_USE
#define DEF___FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f874197a-bf19-5482-9ab1-34923de6738d"))
ITypedEventHandler<IInspectable*, ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundRevokedEventArgs*> : ITypedEventHandler_impl<IInspectable*, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundRevokedEventArgs*, ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundRevokedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Object, Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<IInspectable*, ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundRevokedEventArgs*> __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_t;
#define __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    typedef enum ExtendedExecutionForegroundReason : int ExtendedExecutionForegroundReason;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    typedef enum ExtendedExecutionForegroundRevokedReason : int ExtendedExecutionForegroundRevokedReason;
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    enum ExtendedExecutionForegroundReason : int
                    {
                        ExtendedExecutionForegroundReason_Unspecified = 0,
                        ExtendedExecutionForegroundReason_SavingData = 1,
                        ExtendedExecutionForegroundReason_BackgroundAudio = 2,
                        ExtendedExecutionForegroundReason_Unconstrained = 3,
                    };
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    enum ExtendedExecutionForegroundResult : int
                    {
                        ExtendedExecutionForegroundResult_Allowed = 0,
                        ExtendedExecutionForegroundResult_Denied = 1,
                    };
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    enum ExtendedExecutionForegroundRevokedReason : int
                    {
                        ExtendedExecutionForegroundRevokedReason_Resumed = 0,
                        ExtendedExecutionForegroundRevokedReason_SystemPolicy = 1,
                    };
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ExtendedExecution_Foreground_IExtendedExecutionForegroundRevokedEventArgs[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    MIDL_INTERFACE("b07cd940-9557-aea4-2c99-bdd56d9be461")
                    IExtendedExecutionForegroundRevokedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundRevokedReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IExtendedExecutionForegroundRevokedEventArgs = __uuidof(IExtendedExecutionForegroundRevokedEventArgs);
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ExtendedExecution_Foreground_IExtendedExecutionForegroundSession[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace ExtendedExecution {
                namespace Foreground {
                    MIDL_INTERFACE("fbf440e1-9d10-4201-b01e-c83275296f2e")
                    IExtendedExecutionForegroundSession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Revoked(
                            __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Revoked(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestExtensionAsync(
                            __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundReason* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Reason(
                            ABI::Windows::ApplicationModel::ExtendedExecution::Foreground::ExtendedExecutionForegroundReason value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IExtendedExecutionForegroundSession = __uuidof(IExtendedExecutionForegroundSession);
                } /* Foreground */
            } /* ExtendedExecution */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession;

#endif // ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundResult __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs;

typedef struct __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* This,
        IInspectable* sender,
        __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgsVtbl;

interface __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundReason __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundReason;

typedef enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundRevokedReason __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundRevokedReason;

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundReason
{
    ExtendedExecutionForegroundReason_Unspecified = 0,
    ExtendedExecutionForegroundReason_SavingData = 1,
    ExtendedExecutionForegroundReason_BackgroundAudio = 2,
    ExtendedExecutionForegroundReason_Unconstrained = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundResult
{
    ExtendedExecutionForegroundResult_Allowed = 0,
    ExtendedExecutionForegroundResult_Denied = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundRevokedReason
{
    ExtendedExecutionForegroundRevokedReason_Resumed = 0,
    ExtendedExecutionForegroundRevokedReason_SystemPolicy = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ExtendedExecution_Foreground_IExtendedExecutionForegroundRevokedEventArgs[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundRevokedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundRevokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ExtendedExecution_Foreground_IExtendedExecutionForegroundSession[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession";
typedef struct __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* add_Revoked)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        __FITypedEventHandler_2_IInspectable_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundRevokedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Revoked)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* RequestExtensionAsync)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CExtendedExecution__CForeground__CExtendedExecutionForegroundResult** operation);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundReason* value);
    HRESULT (STDMETHODCALLTYPE* put_Reason)(__x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession* This,
        enum __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CExtendedExecutionForegroundReason value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSessionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_add_Revoked(This, handler, token) \
    ((This)->lpVtbl->add_Revoked(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_remove_Revoked(This, token) \
    ((This)->lpVtbl->remove_Revoked(This, token))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_RequestExtensionAsync(This, operation) \
    ((This)->lpVtbl->RequestExtensionAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#define __x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_put_Reason(This, value) \
    ((This)->lpVtbl->put_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CExtendedExecution_CForeground_CIExtendedExecutionForegroundSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundRevokedEventArgs[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_ExtendedExecution_Foreground_ExtendedExecutionForegroundSession[] = L"Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
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
#endif // __windows2Eapplicationmodel2Eextendedexecution2Eforeground_p_h__

#endif // __windows2Eapplicationmodel2Eextendedexecution2Eforeground_h__
