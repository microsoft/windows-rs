
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
#ifndef __windows2Esecurity2Ecredentials2Eui_h__
#define __windows2Esecurity2Ecredentials2Eui_h__
#ifndef __windows2Esecurity2Ecredentials2Eui_p_h__
#define __windows2Esecurity2Ecredentials2Eui_p_h__


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
#include "Windows.Storage.Streams.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    interface ICredentialPickerOptions;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions ABI::Windows::Security::Credentials::UI::ICredentialPickerOptions

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    interface ICredentialPickerResults;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults ABI::Windows::Security::Credentials::UI::ICredentialPickerResults

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    interface ICredentialPickerStatics;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics ABI::Windows::Security::Credentials::UI::ICredentialPickerStatics

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    interface IUserConsentVerifierStatics;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics ABI::Windows::Security::Credentials::UI::IUserConsentVerifierStatics

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    class CredentialPickerResults;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("83fd09a4-21fd-5c01-9cf6-8293b2929a9f"))
IAsyncOperation<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*, ABI::Windows::Security::Credentials::UI::ICredentialPickerResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.UI.CredentialPickerResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("091a96b8-52a6-523a-8c94-7f9c11c6dae5"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*, ABI::Windows::Security::Credentials::UI::ICredentialPickerResults*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.UI.CredentialPickerResults>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Credentials::UI::CredentialPickerResults*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    typedef enum UserConsentVerificationResult : int UserConsentVerificationResult;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fd596ffd-2318-558f-9dbe-d21df43764a5"))
IAsyncOperation<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult> : IAsyncOperation_impl<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.UI.UserConsentVerificationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0cffc6c9-4c2b-5cd4-b38c-7b8df3ff5afb"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.UI.UserConsentVerificationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Credentials::UI::UserConsentVerificationResult> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    typedef enum UserConsentVerifierAvailability : int UserConsentVerifierAvailability;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ddd384f3-d818-5d83-ab4b-32119c28587c"))
IAsyncOperation<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability> : IAsyncOperation_impl<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Credentials.UI.UserConsentVerifierAvailability>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability> __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("28988174-ace2-5c15-a0df-580a26d94294"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Credentials.UI.UserConsentVerifierAvailability>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Security::Credentials::UI::UserConsentVerifierAvailability> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    typedef enum AuthenticationProtocol : int AuthenticationProtocol;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    typedef enum CredentialSaveOption : int CredentialSaveOption;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    class CredentialPickerOptions;
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Credentials.UI.AuthenticationProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    enum AuthenticationProtocol : int
                    {
                        AuthenticationProtocol_Basic = 0,
                        AuthenticationProtocol_Digest = 1,
                        AuthenticationProtocol_Ntlm = 2,
                        AuthenticationProtocol_Kerberos = 3,
                        AuthenticationProtocol_Negotiate = 4,
                        AuthenticationProtocol_CredSsp = 5,
                        AuthenticationProtocol_Custom = 6,
                    };
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.CredentialSaveOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    enum CredentialSaveOption : int
                    {
                        CredentialSaveOption_Unselected = 0,
                        CredentialSaveOption_Selected = 1,
                        CredentialSaveOption_Hidden = 2,
                    };
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.UserConsentVerificationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    enum UserConsentVerificationResult : int
                    {
                        UserConsentVerificationResult_Verified = 0,
                        UserConsentVerificationResult_DeviceNotPresent = 1,
                        UserConsentVerificationResult_NotConfiguredForUser = 2,
                        UserConsentVerificationResult_DisabledByPolicy = 3,
                        UserConsentVerificationResult_DeviceBusy = 4,
                        UserConsentVerificationResult_RetriesExhausted = 5,
                        UserConsentVerificationResult_Canceled = 6,
                    };
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.UserConsentVerifierAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    enum UserConsentVerifierAvailability : int
                    {
                        UserConsentVerifierAvailability_Available = 0,
                        UserConsentVerifierAvailability_DeviceNotPresent = 1,
                        UserConsentVerifierAvailability_NotConfiguredForUser = 2,
                        UserConsentVerifierAvailability_DisabledByPolicy = 3,
                        UserConsentVerifierAvailability_DeviceBusy = 4,
                    };
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPickerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerOptions[] = L"Windows.Security.Credentials.UI.ICredentialPickerOptions";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    MIDL_INTERFACE("965a0b4c-95fa-467f-992b-0b22e5859bf6")
                    ICredentialPickerOptions : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_Caption(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Caption(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Message(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Message(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ErrorCode(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TargetName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TargetName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AuthenticationProtocol(
                            ABI::Windows::Security::Credentials::UI::AuthenticationProtocol value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthenticationProtocol(
                            ABI::Windows::Security::Credentials::UI::AuthenticationProtocol* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CustomAuthenticationProtocol(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CustomAuthenticationProtocol(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PreviousCredential(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PreviousCredential(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AlwaysDisplayDialog(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AlwaysDisplayDialog(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CallerSavesCredential(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CallerSavesCredential(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CredentialSaveOption(
                            ABI::Windows::Security::Credentials::UI::CredentialSaveOption value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialSaveOption(
                            ABI::Windows::Security::Credentials::UI::CredentialSaveOption* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICredentialPickerOptions = __uuidof(ICredentialPickerOptions);
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPickerResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerResults[] = L"Windows.Security.Credentials.UI.ICredentialPickerResults";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    MIDL_INTERFACE("1948f99a-cc30-410c-9c38-cc0884c5b3d7")
                    ICredentialPickerResults : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialSaveOption(
                            ABI::Windows::Security::Credentials::UI::CredentialSaveOption* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialSaved(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Credential(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialDomainName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialUserName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CredentialPassword(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICredentialPickerResults = __uuidof(ICredentialPickerResults);
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerStatics[] = L"Windows.Security.Credentials.UI.ICredentialPickerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    MIDL_INTERFACE("aa3a5c73-c9ea-4782-99fb-e6d7e938e12d")
                    ICredentialPickerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE PickWithOptionsAsync(
                            ABI::Windows::Security::Credentials::UI::ICredentialPickerOptions* options,
                            __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PickWithMessageAsync(
                            HSTRING targetName,
                            HSTRING message,
                            __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PickWithCaptionAsync(
                            HSTRING targetName,
                            HSTRING message,
                            HSTRING caption,
                            __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICredentialPickerStatics = __uuidof(ICredentialPickerStatics);
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.IUserConsentVerifierStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.UserConsentVerifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_IUserConsentVerifierStatics[] = L"Windows.Security.Credentials.UI.IUserConsentVerifierStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                namespace UI {
                    MIDL_INTERFACE("af4f3f91-564c-4ddc-b8b5-973447627c65")
                    IUserConsentVerifierStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CheckAvailabilityAsync(
                            __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestVerificationAsync(
                            HSTRING message,
                            __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserConsentVerifierStatics = __uuidof(IUserConsentVerifierStatics);
                } /* UI */
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.UI.ICredentialPickerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPicker_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPicker[] = L"Windows.Security.Credentials.UI.CredentialPicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPickerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.UI.ICredentialPickerOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPickerOptions[] = L"Windows.Security.Credentials.UI.CredentialPickerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPickerResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.UI.ICredentialPickerResults ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerResults_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPickerResults[] = L"Windows.Security.Credentials.UI.CredentialPickerResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.UserConsentVerifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.UI.IUserConsentVerifierStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_UserConsentVerifier_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_UserConsentVerifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_UserConsentVerifier[] = L"Windows.Security.Credentials.UI.UserConsentVerifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerificationResult __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerificationResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerificationResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerifierAvailability __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerifierAvailability;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerifierAvailability* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailabilityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CAuthenticationProtocol __x_ABI_CWindows_CSecurity_CCredentials_CUI_CAuthenticationProtocol;

typedef enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption;

/*
 *
 * Struct Windows.Security.Credentials.UI.AuthenticationProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CAuthenticationProtocol
{
    AuthenticationProtocol_Basic = 0,
    AuthenticationProtocol_Digest = 1,
    AuthenticationProtocol_Ntlm = 2,
    AuthenticationProtocol_Kerberos = 3,
    AuthenticationProtocol_Negotiate = 4,
    AuthenticationProtocol_CredSsp = 5,
    AuthenticationProtocol_Custom = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.CredentialSaveOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption
{
    CredentialSaveOption_Unselected = 0,
    CredentialSaveOption_Selected = 1,
    CredentialSaveOption_Hidden = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.UserConsentVerificationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerificationResult
{
    UserConsentVerificationResult_Verified = 0,
    UserConsentVerificationResult_DeviceNotPresent = 1,
    UserConsentVerificationResult_NotConfiguredForUser = 2,
    UserConsentVerificationResult_DisabledByPolicy = 3,
    UserConsentVerificationResult_DeviceBusy = 4,
    UserConsentVerificationResult_RetriesExhausted = 5,
    UserConsentVerificationResult_Canceled = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Credentials.UI.UserConsentVerifierAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CUserConsentVerifierAvailability
{
    UserConsentVerifierAvailability_Available = 0,
    UserConsentVerifierAvailability_DeviceNotPresent = 1,
    UserConsentVerifierAvailability_NotConfiguredForUser = 2,
    UserConsentVerifierAvailability_DisabledByPolicy = 3,
    UserConsentVerifierAvailability_DeviceBusy = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPickerOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerOptions[] = L"Windows.Security.Credentials.UI.ICredentialPickerOptions";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Caption)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Caption)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Message)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ErrorCode)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TargetName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_TargetName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AuthenticationProtocol)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CAuthenticationProtocol value);
    HRESULT (STDMETHODCALLTYPE* get_AuthenticationProtocol)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CAuthenticationProtocol* value);
    HRESULT (STDMETHODCALLTYPE* put_CustomAuthenticationProtocol)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CustomAuthenticationProtocol)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_PreviousCredential)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_PreviousCredential)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_AlwaysDisplayDialog)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AlwaysDisplayDialog)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CallerSavesCredential)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CallerSavesCredential)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CredentialSaveOption)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialSaveOption)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptionsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_Caption(This, value) \
    ((This)->lpVtbl->put_Caption(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_Caption(This, value) \
    ((This)->lpVtbl->get_Caption(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_Message(This, value) \
    ((This)->lpVtbl->put_Message(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_ErrorCode(This, value) \
    ((This)->lpVtbl->put_ErrorCode(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_TargetName(This, value) \
    ((This)->lpVtbl->put_TargetName(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_TargetName(This, value) \
    ((This)->lpVtbl->get_TargetName(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_AuthenticationProtocol(This, value) \
    ((This)->lpVtbl->put_AuthenticationProtocol(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_AuthenticationProtocol(This, value) \
    ((This)->lpVtbl->get_AuthenticationProtocol(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_CustomAuthenticationProtocol(This, value) \
    ((This)->lpVtbl->put_CustomAuthenticationProtocol(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_CustomAuthenticationProtocol(This, value) \
    ((This)->lpVtbl->get_CustomAuthenticationProtocol(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_PreviousCredential(This, value) \
    ((This)->lpVtbl->put_PreviousCredential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_PreviousCredential(This, value) \
    ((This)->lpVtbl->get_PreviousCredential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_AlwaysDisplayDialog(This, value) \
    ((This)->lpVtbl->put_AlwaysDisplayDialog(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_AlwaysDisplayDialog(This, value) \
    ((This)->lpVtbl->get_AlwaysDisplayDialog(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_CallerSavesCredential(This, value) \
    ((This)->lpVtbl->put_CallerSavesCredential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_CallerSavesCredential(This, value) \
    ((This)->lpVtbl->get_CallerSavesCredential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_put_CredentialSaveOption(This, value) \
    ((This)->lpVtbl->put_CredentialSaveOption(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_get_CredentialSaveOption(This, value) \
    ((This)->lpVtbl->get_CredentialSaveOption(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPickerResults
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerResults[] = L"Windows.Security.Credentials.UI.ICredentialPickerResults";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResultsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialSaveOption)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        enum __x_ABI_CWindows_CSecurity_CCredentials_CUI_CCredentialSaveOption* value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialSaved)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Credential)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialDomainName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialUserName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialPassword)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResultsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResultsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_CredentialSaveOption(This, value) \
    ((This)->lpVtbl->get_CredentialSaveOption(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_CredentialSaved(This, value) \
    ((This)->lpVtbl->get_CredentialSaved(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_Credential(This, value) \
    ((This)->lpVtbl->get_Credential(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_CredentialDomainName(This, value) \
    ((This)->lpVtbl->get_CredentialDomainName(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_CredentialUserName(This, value) \
    ((This)->lpVtbl->get_CredentialUserName(This, value))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_get_CredentialPassword(This, value) \
    ((This)->lpVtbl->get_CredentialPassword(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerResults_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.ICredentialPickerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.CredentialPicker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_ICredentialPickerStatics[] = L"Windows.Security.Credentials.UI.ICredentialPickerStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PickWithOptionsAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerOptions* options,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation);
    HRESULT (STDMETHODCALLTYPE* PickWithMessageAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        HSTRING targetName,
        HSTRING message,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation);
    HRESULT (STDMETHODCALLTYPE* PickWithCaptionAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics* This,
        HSTRING targetName,
        HSTRING message,
        HSTRING caption,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CCredentialPickerResults** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_PickWithOptionsAsync(This, options, operation) \
    ((This)->lpVtbl->PickWithOptionsAsync(This, options, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_PickWithMessageAsync(This, targetName, message, operation) \
    ((This)->lpVtbl->PickWithMessageAsync(This, targetName, message, operation))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_PickWithCaptionAsync(This, targetName, message, caption, operation) \
    ((This)->lpVtbl->PickWithCaptionAsync(This, targetName, message, caption, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CICredentialPickerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Credentials.UI.IUserConsentVerifierStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Credentials.UI.UserConsentVerifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Credentials_UI_IUserConsentVerifierStatics[] = L"Windows.Security.Credentials.UI.IUserConsentVerifierStatics";
typedef struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CheckAvailabilityAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerifierAvailability** result);
    HRESULT (STDMETHODCALLTYPE* RequestVerificationAsync)(__x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics* This,
        HSTRING message,
        __FIAsyncOperation_1_Windows__CSecurity__CCredentials__CUI__CUserConsentVerificationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_CheckAvailabilityAsync(This, result) \
    ((This)->lpVtbl->CheckAvailabilityAsync(This, result))

#define __x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_RequestVerificationAsync(This, message, result) \
    ((This)->lpVtbl->RequestVerificationAsync(This, message, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CCredentials_CUI_CIUserConsentVerifierStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPicker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.UI.ICredentialPickerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPicker_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPicker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPicker[] = L"Windows.Security.Credentials.UI.CredentialPicker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPickerOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.UI.ICredentialPickerOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPickerOptions[] = L"Windows.Security.Credentials.UI.CredentialPickerOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.CredentialPickerResults
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Credentials.UI.ICredentialPickerResults ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerResults_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_CredentialPickerResults_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_CredentialPickerResults[] = L"Windows.Security.Credentials.UI.CredentialPickerResults";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Credentials.UI.UserConsentVerifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Credentials.UI.IUserConsentVerifierStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Credentials_UI_UserConsentVerifier_DEFINED
#define RUNTIMECLASS_Windows_Security_Credentials_UI_UserConsentVerifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Credentials_UI_UserConsentVerifier[] = L"Windows.Security.Credentials.UI.UserConsentVerifier";
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
#endif // __windows2Esecurity2Ecredentials2Eui_p_h__

#endif // __windows2Esecurity2Ecredentials2Eui_h__
