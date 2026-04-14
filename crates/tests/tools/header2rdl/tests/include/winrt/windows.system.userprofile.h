
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
#ifndef __windows2Esystem2Euserprofile_h__
#define __windows2Esystem2Euserprofile_h__
#ifndef __windows2Esystem2Euserprofile_p_h__
#define __windows2Esystem2Euserprofile_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION)
#define WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION)
#define WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Globalization.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IAdvertisingManagerForUser;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser ABI::Windows::System::UserProfile::IAdvertisingManagerForUser

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IAdvertisingManagerStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics ABI::Windows::System::UserProfile::IAdvertisingManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IAdvertisingManagerStatics2;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2 ABI::Windows::System::UserProfile::IAdvertisingManagerStatics2

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IAssignedAccessSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings ABI::Windows::System::UserProfile::IAssignedAccessSettings

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IAssignedAccessSettingsStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics ABI::Windows::System::UserProfile::IAssignedAccessSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IDiagnosticsSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings ABI::Windows::System::UserProfile::IDiagnosticsSettings

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IDiagnosticsSettingsStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics ABI::Windows::System::UserProfile::IDiagnosticsSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IFirstSignInSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings ABI::Windows::System::UserProfile::IFirstSignInSettings

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IFirstSignInSettingsStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics ABI::Windows::System::UserProfile::IFirstSignInSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IGlobalizationPreferencesForUser;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser ABI::Windows::System::UserProfile::IGlobalizationPreferencesForUser

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IGlobalizationPreferencesStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics ABI::Windows::System::UserProfile::IGlobalizationPreferencesStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IGlobalizationPreferencesStatics2;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2 ABI::Windows::System::UserProfile::IGlobalizationPreferencesStatics2

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IGlobalizationPreferencesStatics3;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3 ABI::Windows::System::UserProfile::IGlobalizationPreferencesStatics3

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface ILockScreenImageFeedStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics ABI::Windows::System::UserProfile::ILockScreenImageFeedStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface ILockScreenStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics ABI::Windows::System::UserProfile::ILockScreenStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IUserInformationStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics ABI::Windows::System::UserProfile::IUserInformationStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IUserProfilePersonalizationSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings ABI::Windows::System::UserProfile::IUserProfilePersonalizationSettings

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                interface IUserProfilePersonalizationSettingsStatics;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics ABI::Windows::System::UserProfile::IUserProfilePersonalizationSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE
#define DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("641cb9dd-a28d-59e2-b8db-a227eda6cf2e"))
IAsyncOperation<ABI::Windows::Foundation::Uri*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Foundation::Uri*> __FIAsyncOperation_1_Windows__CFoundation__CUri_t;
#define __FIAsyncOperation_1_Windows__CFoundation__CUri ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CFoundation__CUri_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad46f1cc-2bb0-585c-9885-03c2780d4d58"))
IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Uri*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Foundation::Uri*> __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                typedef enum SetAccountPictureResult : int SetAccountPictureResult;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6809e406-6d3b-5164-8f32-b845b0781405"))
IAsyncOperation<enum ABI::Windows::System::UserProfile::SetAccountPictureResult> : IAsyncOperation_impl<enum ABI::Windows::System::UserProfile::SetAccountPictureResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.UserProfile.SetAccountPictureResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::System::UserProfile::SetAccountPictureResult> __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_t;
#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE */

#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("603f3e31-7a51-518c-9280-c188ea4213d8"))
IAsyncOperationCompletedHandler<enum ABI::Windows::System::UserProfile::SetAccountPictureResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::System::UserProfile::SetAccountPictureResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.UserProfile.SetAccountPictureResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::System::UserProfile::SetAccountPictureResult> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_USE */

#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                typedef enum SetImageFeedResult : int SetImageFeedResult;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5361bfc9-0740-544a-9797-1ffe5e73c54e"))
IAsyncOperation<enum ABI::Windows::System::UserProfile::SetImageFeedResult> : IAsyncOperation_impl<enum ABI::Windows::System::UserProfile::SetImageFeedResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.UserProfile.SetImageFeedResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::System::UserProfile::SetImageFeedResult> __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_t;
#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE */

#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f214731a-1305-5b44-932c-af9a1e4d78c9"))
IAsyncOperationCompletedHandler<enum ABI::Windows::System::UserProfile::SetImageFeedResult> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::System::UserProfile::SetImageFeedResult>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.UserProfile.SetImageFeedResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::System::UserProfile::SetImageFeedResult> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_USE */

#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */



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



#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


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
        namespace Globalization {
            typedef enum DayOfWeek : int DayOfWeek;
        } /* Globalization */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                typedef enum AccountPictureKind : int AccountPictureKind;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class AdvertisingManagerForUser;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class AssignedAccessSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class DiagnosticsSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class FirstSignInSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class GlobalizationPreferencesForUser;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                class UserProfilePersonalizationSettings;
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.UserProfile.AccountPictureKind
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                enum
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                AccountPictureKind : int
                {
                    AccountPictureKind_SmallImage = 0,
                    AccountPictureKind_LargeImage = 1,
                    AccountPictureKind_Video = 2,
                };
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.UserProfile.SetAccountPictureResult
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                enum
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                SetAccountPictureResult : int
                {
                    SetAccountPictureResult_Success = 0,
                    SetAccountPictureResult_ChangeDisabled = 1,
                    SetAccountPictureResult_LargeOrDynamicError = 2,
                    SetAccountPictureResult_VideoFrameSizeError = 3,
                    SetAccountPictureResult_FileSizeError = 4,
                    SetAccountPictureResult_Failure = 5,
                };
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.UserProfile.SetImageFeedResult
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                enum SetImageFeedResult : int
                {
                    SetImageFeedResult_Success = 0,
                    SetImageFeedResult_ChangeDisabled = 1,
                    SetImageFeedResult_UserCanceled = 2,
                };
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerForUser[] = L"Windows.System.UserProfile.IAdvertisingManagerForUser";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("928bf3d0-cf7c-4ab0-a7dc-6dc5bcd44252")
                IAdvertisingManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AdvertisingId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvertisingManagerForUser = __uuidof(IAdvertisingManagerForUser);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerStatics[] = L"Windows.System.UserProfile.IAdvertisingManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("add3468c-a273-48cb-b346-3544522d5581")
                IAdvertisingManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AdvertisingId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvertisingManagerStatics = __uuidof(IAdvertisingManagerStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerStatics2[] = L"Windows.System.UserProfile.IAdvertisingManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("dd0947af-1a6d-46b0-95bc-f3f9d6beb9fb")
                IAdvertisingManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::UserProfile::IAdvertisingManagerForUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvertisingManagerStatics2 = __uuidof(IAdvertisingManagerStatics2);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.UserProfile.IAssignedAccessSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AssignedAccessSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAssignedAccessSettings[] = L"Windows.System.UserProfile.IAssignedAccessSettings";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("1bc57f1c-e971-5757-b8e0-512f8b8c46d2")
                IAssignedAccessSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSingleAppKioskMode(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAssignedAccessSettings = __uuidof(IAssignedAccessSettings);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.UserProfile.IAssignedAccessSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AssignedAccessSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAssignedAccessSettingsStatics[] = L"Windows.System.UserProfile.IAssignedAccessSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("34a81d0d-8a29-5ef3-a7be-618e6ac3bd01")
                IAssignedAccessSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::System::UserProfile::IAssignedAccessSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::UserProfile::IAssignedAccessSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAssignedAccessSettingsStatics = __uuidof(IAssignedAccessSettingsStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.UserProfile.IDiagnosticsSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.DiagnosticsSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IDiagnosticsSettings[] = L"Windows.System.UserProfile.IDiagnosticsSettings";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("e5e9eccd-2711-44e0-973c-491d78048d24")
                IDiagnosticsSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanUseDiagnosticsToTailorExperiences(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticsSettings = __uuidof(IDiagnosticsSettings);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.UserProfile.IDiagnosticsSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.DiagnosticsSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IDiagnosticsSettingsStatics[] = L"Windows.System.UserProfile.IDiagnosticsSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("72d2e80f-5390-4793-990b-3ccc7d6ac9c8")
                IDiagnosticsSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::System::UserProfile::IDiagnosticsSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::UserProfile::IDiagnosticsSettings** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDiagnosticsSettingsStatics = __uuidof(IDiagnosticsSettingsStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.UserProfile.IFirstSignInSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.FirstSignInSettings
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IFirstSignInSettings[] = L"Windows.System.UserProfile.IFirstSignInSettings";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("3e945153-3a5e-452e-a601-f5baad2a4870")
                IFirstSignInSettings : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IFirstSignInSettings = __uuidof(IFirstSignInSettings);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IFirstSignInSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.FirstSignInSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IFirstSignInSettingsStatics[] = L"Windows.System.UserProfile.IFirstSignInSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("1ce18f0f-1c41-4ea0-b7a2-6f0c1c7e8438")
                IFirstSignInSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::System::UserProfile::IFirstSignInSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFirstSignInSettingsStatics = __uuidof(IFirstSignInSettingsStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferencesForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesForUser[] = L"Windows.System.UserProfile.IGlobalizationPreferencesForUser";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("150f0795-4f6e-40ba-a010-e27d81bda7f5")
                IGlobalizationPreferencesForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Calendars(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clocks(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Currencies(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Languages(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HomeGeographicRegion(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WeekStartsOn(
                        ABI::Windows::Globalization::DayOfWeek* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGlobalizationPreferencesForUser = __uuidof(IGlobalizationPreferencesForUser);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("01bf4326-ed37-4e96-b0e9-c1340d1ea158")
                IGlobalizationPreferencesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Calendars(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Clocks(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Currencies(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Languages(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HomeGeographicRegion(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WeekStartsOn(
                        ABI::Windows::Globalization::DayOfWeek* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGlobalizationPreferencesStatics = __uuidof(IGlobalizationPreferencesStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics2[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("fcce85f1-4300-4cd0-9cac-1a8e7b7e18f4")
                IGlobalizationPreferencesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TrySetHomeGeographicRegion(
                        HSTRING region,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetLanguages(
                        __FIIterable_1_HSTRING* languageTags,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGlobalizationPreferencesStatics2 = __uuidof(IGlobalizationPreferencesStatics2);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics3[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics3";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("1e059733-35f5-40d8-b9e8-aef3ef856fce")
                IGlobalizationPreferencesStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::UserProfile::IGlobalizationPreferencesForUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IGlobalizationPreferencesStatics3 = __uuidof(IGlobalizationPreferencesStatics3);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.UserProfile.ILockScreenImageFeedStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.LockScreen
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_ILockScreenImageFeedStatics[] = L"Windows.System.UserProfile.ILockScreenImageFeedStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("2c0d73f6-03a9-41a6-9b01-495251ff51d5")
                ILockScreenImageFeedStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestSetImageFeedAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* syndicationFeedUri,
                        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryRemoveImageFeed(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenImageFeedStatics = __uuidof(ILockScreenImageFeedStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.ILockScreenStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.LockScreen
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_ILockScreenStatics[] = L"Windows.System.UserProfile.ILockScreenStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("3ee9d3ad-b607-40ae-b426-7631d9821269")
                ILockScreenStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalImageFile(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetImageStream(
                        ABI::Windows::Storage::Streams::IRandomAccessStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetImageFileAsync(
                        ABI::Windows::Storage::IStorageFile* value,
                        ABI::Windows::Foundation::IAsyncAction** Operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetImageStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* value,
                        ABI::Windows::Foundation::IAsyncAction** Operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenStatics = __uuidof(ILockScreenStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserInformationStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserInformation
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserInformationStatics[] = L"Windows.System.UserProfile.IUserInformationStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("77f3a910-48fa-489c-934e-2ae85ba8f772")
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                IUserInformationStatics : public IInspectable
                {
                public:
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_AccountPictureChangeEnabled(
                        boolean* value
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE get_NameAccessAllowed(
                        boolean* value
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetAccountPicture(
                        ABI::Windows::System::UserProfile::AccountPictureKind kind,
                        ABI::Windows::Storage::IStorageFile** storageFile
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE SetAccountPictureAsync(
                        ABI::Windows::Storage::IStorageFile* image,
                        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE SetAccountPicturesAsync(
                        ABI::Windows::Storage::IStorageFile* smallImage,
                        ABI::Windows::Storage::IStorageFile* largeImage,
                        ABI::Windows::Storage::IStorageFile* video,
                        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE SetAccountPictureFromStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* image,
                        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE SetAccountPicturesFromStreamsAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* smallImage,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* largeImage,
                        ABI::Windows::Storage::Streams::IRandomAccessStream* video,
                        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE add_AccountPictureChanged(
                        __FIEventHandler_1_IInspectable* changeHandler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE remove_AccountPictureChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetDisplayNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetFirstNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetLastNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetPrincipalNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetSessionInitiationProtocolUriAsync(
                        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation
                        ) = 0;
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
                    virtual HRESULT STDMETHODCALLTYPE GetDomainNameAsync(
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserInformationStatics = __uuidof(IUserInformationStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserProfilePersonalizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserProfilePersonalizationSettings[] = L"Windows.System.UserProfile.IUserProfilePersonalizationSettings";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("8ceddab4-7998-46d5-8dd3-184f1c5f9ab9")
                IUserProfilePersonalizationSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TrySetLockScreenImageAsync(
                        ABI::Windows::Storage::IStorageFile* imageFile,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TrySetWallpaperImageAsync(
                        ABI::Windows::Storage::IStorageFile* imageFile,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserProfilePersonalizationSettings = __uuidof(IUserProfilePersonalizationSettings);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserProfilePersonalizationSettingsStatics[] = L"Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace UserProfile {
                MIDL_INTERFACE("91acb841-5037-454b-9883-bb772d08dd16")
                IUserProfilePersonalizationSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Current(
                        ABI::Windows::System::UserProfile::IUserProfilePersonalizationSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserProfilePersonalizationSettingsStatics = __uuidof(IUserProfilePersonalizationSettingsStatics);
            } /* UserProfile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.AdvertisingManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IAdvertisingManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IAdvertisingManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManager_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AdvertisingManager[] = L"Windows.System.UserProfile.AdvertisingManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.AdvertisingManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IAdvertisingManagerForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AdvertisingManagerForUser[] = L"Windows.System.UserProfile.AdvertisingManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.UserProfile.AssignedAccessSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IAssignedAccessSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IAssignedAccessSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AssignedAccessSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AssignedAccessSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AssignedAccessSettings[] = L"Windows.System.UserProfile.AssignedAccessSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.UserProfile.DiagnosticsSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IDiagnosticsSettingsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IDiagnosticsSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_DiagnosticsSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_DiagnosticsSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_DiagnosticsSettings[] = L"Windows.System.UserProfile.DiagnosticsSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.UserProfile.FirstSignInSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IFirstSignInSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IFirstSignInSettings ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_FirstSignInSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_FirstSignInSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_FirstSignInSettings[] = L"Windows.System.UserProfile.FirstSignInSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.GlobalizationPreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferences_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferences_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_GlobalizationPreferences[] = L"Windows.System.UserProfile.GlobalizationPreferences";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.GlobalizationPreferencesForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IGlobalizationPreferencesForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferencesForUser_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferencesForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_GlobalizationPreferencesForUser[] = L"Windows.System.UserProfile.GlobalizationPreferencesForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.UserProfile.LockScreen
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.ILockScreenStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileLockScreenContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.ILockScreenImageFeedStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileLockScreenContract API contract
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_LockScreen_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_LockScreen_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_LockScreen[] = L"Windows.System.UserProfile.LockScreen";
#endif
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.UserInformation
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IUserInformationStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileContract API contract
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_UserInformation_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_UserInformation_DEFINED
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_UserInformation[] = L"Windows.System.UserProfile.UserInformation";
#endif
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IUserProfilePersonalizationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_UserProfilePersonalizationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_UserProfilePersonalizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_UserProfilePersonalizationSettings[] = L"Windows.System.UserProfile.UserProfilePersonalizationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2 __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2 __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3 __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CFoundation__CUri __FIAsyncOperation_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CFoundation__CUri;

typedef struct __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl;

interface __FIAsyncOperation_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CFoundation__CUri_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSystem_CUserProfile_CSetAccountPictureResult __x_ABI_CWindows_CSystem_CUserProfile_CSetAccountPictureResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult;

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        enum __x_ABI_CWindows_CSystem_CUserProfile_CSetAccountPictureResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* This,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSystem_CUserProfile_CSetImageFeedResult __x_ABI_CWindows_CSystem_CUserProfile_CSetImageFeedResult;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult;

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        enum __x_ABI_CWindows_CSystem_CUserProfile_CSetImageFeedResult* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* This,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CUserProfile__CSetImageFeedResult_INTERFACE_DEFINED__
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

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

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGlobalization_CDayOfWeek __x_ABI_CWindows_CGlobalization_CDayOfWeek;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CUserProfile_CAccountPictureKind __x_ABI_CWindows_CSystem_CUserProfile_CAccountPictureKind;

/*
 *
 * Struct Windows.System.UserProfile.AccountPictureKind
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CSystem_CUserProfile_CAccountPictureKind
{
    AccountPictureKind_SmallImage = 0,
    AccountPictureKind_LargeImage = 1,
    AccountPictureKind_Video = 2,
};
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.UserProfile.SetAccountPictureResult
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CSystem_CUserProfile_CSetAccountPictureResult
{
    SetAccountPictureResult_Success = 0,
    SetAccountPictureResult_ChangeDisabled = 1,
    SetAccountPictureResult_LargeOrDynamicError = 2,
    SetAccountPictureResult_VideoFrameSizeError = 3,
    SetAccountPictureResult_FileSizeError = 4,
    SetAccountPictureResult_Failure = 5,
};
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.UserProfile.SetImageFeedResult
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CUserProfile_CSetImageFeedResult
{
    SetImageFeedResult_Success = 0,
    SetImageFeedResult_ChangeDisabled = 1,
    SetImageFeedResult_UserCanceled = 2,
};
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerForUser[] = L"Windows.System.UserProfile.IAdvertisingManagerForUser";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisingId)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUserVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_get_AdvertisingId(This, value) \
    ((This)->lpVtbl->get_AdvertisingId(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerStatics[] = L"Windows.System.UserProfile.IAdvertisingManagerStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdvertisingId)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_get_AdvertisingId(This, value) \
    ((This)->lpVtbl->get_AdvertisingId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IAdvertisingManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AdvertisingManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAdvertisingManagerStatics2[] = L"Windows.System.UserProfile.IAdvertisingManagerStatics2";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerForUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_GetForUser(This, user, value) \
    ((This)->lpVtbl->GetForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAdvertisingManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.UserProfile.IAssignedAccessSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AssignedAccessSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAssignedAccessSettings[] = L"Windows.System.UserProfile.IAssignedAccessSettings";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSingleAppKioskMode)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_get_IsSingleAppKioskMode(This, value) \
    ((This)->lpVtbl->get_IsSingleAppKioskMode(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.UserProfile.IAssignedAccessSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.AssignedAccessSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IAssignedAccessSettingsStatics[] = L"Windows.System.UserProfile.IAssignedAccessSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings** result);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIAssignedAccessSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.UserProfile.IDiagnosticsSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.DiagnosticsSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IDiagnosticsSettings[] = L"Windows.System.UserProfile.IDiagnosticsSettings";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanUseDiagnosticsToTailorExperiences)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_get_CanUseDiagnosticsToTailorExperiences(This, value) \
    ((This)->lpVtbl->get_CanUseDiagnosticsToTailorExperiences(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.UserProfile.IDiagnosticsSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.DiagnosticsSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IDiagnosticsSettingsStatics[] = L"Windows.System.UserProfile.IDiagnosticsSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings** value);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_GetForUser(This, user, value) \
    ((This)->lpVtbl->GetForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIDiagnosticsSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.UserProfile.IFirstSignInSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.FirstSignInSettings
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IMapView`2<String, Object>
 *     Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IFirstSignInSettings[] = L"Windows.System.UserProfile.IFirstSignInSettings";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IFirstSignInSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.FirstSignInSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IFirstSignInSettingsStatics[] = L"Windows.System.UserProfile.IFirstSignInSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIFirstSignInSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferencesForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesForUser[] = L"Windows.System.UserProfile.IGlobalizationPreferencesForUser";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* get_Calendars)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Clocks)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Currencies)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_HomeGeographicRegion)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_WeekStartsOn)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser* This,
        enum __x_ABI_CWindows_CGlobalization_CDayOfWeek* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUserVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_Calendars(This, value) \
    ((This)->lpVtbl->get_Calendars(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_Clocks(This, value) \
    ((This)->lpVtbl->get_Clocks(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_Currencies(This, value) \
    ((This)->lpVtbl->get_Currencies(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_HomeGeographicRegion(This, value) \
    ((This)->lpVtbl->get_HomeGeographicRegion(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_get_WeekStartsOn(This, value) \
    ((This)->lpVtbl->get_WeekStartsOn(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Calendars)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Clocks)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Currencies)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Languages)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_HomeGeographicRegion)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_WeekStartsOn)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics* This,
        enum __x_ABI_CWindows_CGlobalization_CDayOfWeek* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_Calendars(This, value) \
    ((This)->lpVtbl->get_Calendars(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_Clocks(This, value) \
    ((This)->lpVtbl->get_Clocks(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_Currencies(This, value) \
    ((This)->lpVtbl->get_Currencies(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_Languages(This, value) \
    ((This)->lpVtbl->get_Languages(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_HomeGeographicRegion(This, value) \
    ((This)->lpVtbl->get_HomeGeographicRegion(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_get_WeekStartsOn(This, value) \
    ((This)->lpVtbl->get_WeekStartsOn(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics2[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics2";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TrySetHomeGeographicRegion)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        HSTRING region,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* TrySetLanguages)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2* This,
        __FIIterable_1_HSTRING* languageTags,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_TrySetHomeGeographicRegion(This, region, result) \
    ((This)->lpVtbl->TrySetHomeGeographicRegion(This, region, result))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_TrySetLanguages(This, languageTags, result) \
    ((This)->lpVtbl->TrySetLanguages(This, languageTags, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.System.UserProfile.IGlobalizationPreferencesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.GlobalizationPreferences
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IGlobalizationPreferencesStatics3[] = L"Windows.System.UserProfile.IGlobalizationPreferencesStatics3";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesForUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3Vtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_GetForUser(This, user, value) \
    ((This)->lpVtbl->GetForUser(This, user, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIGlobalizationPreferencesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.UserProfile.ILockScreenImageFeedStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.LockScreen
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_ILockScreenImageFeedStatics[] = L"Windows.System.UserProfile.ILockScreenImageFeedStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestSetImageFeedAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* syndicationFeedUri,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetImageFeedResult** value);
    HRESULT (STDMETHODCALLTYPE* TryRemoveImageFeed)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_RequestSetImageFeedAsync(This, syndicationFeedUri, value) \
    ((This)->lpVtbl->RequestSetImageFeedAsync(This, syndicationFeedUri, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_TryRemoveImageFeed(This, result) \
    ((This)->lpVtbl->TryRemoveImageFeed(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenImageFeedStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.ILockScreenStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.LockScreen
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_ILockScreenStatics[] = L"Windows.System.UserProfile.ILockScreenStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OriginalImageFile)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* GetImageStream)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** value);
    HRESULT (STDMETHODCALLTYPE* SetImageFileAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** Operation);
    HRESULT (STDMETHODCALLTYPE* SetImageStreamAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** Operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_get_OriginalImageFile(This, value) \
    ((This)->lpVtbl->get_OriginalImageFile(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_GetImageStream(This, value) \
    ((This)->lpVtbl->GetImageStream(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_SetImageFileAsync(This, value, Operation) \
    ((This)->lpVtbl->SetImageFileAsync(This, value, Operation))

#define __x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_SetImageStreamAsync(This, value, Operation) \
    ((This)->lpVtbl->SetImageStreamAsync(This, value, Operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CILockScreenStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserInformationStatics
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserInformation
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserInformationStatics[] = L"Windows.System.UserProfile.IUserInformationStatics";
typedef struct
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_AccountPictureChangeEnabled)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        boolean* value);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* get_NameAccessAllowed)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        boolean* value);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetAccountPicture)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        enum __x_ABI_CWindows_CSystem_CUserProfile_CAccountPictureKind kind,
        __x_ABI_CWindows_CStorage_CIStorageFile** storageFile);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* SetAccountPictureAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* image,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* SetAccountPicturesAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* smallImage,
        __x_ABI_CWindows_CStorage_CIStorageFile* largeImage,
        __x_ABI_CWindows_CStorage_CIStorageFile* video,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* SetAccountPictureFromStreamAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* image,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* SetAccountPicturesFromStreamsAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* smallImage,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* largeImage,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* video,
        __FIAsyncOperation_1_Windows__CSystem__CUserProfile__CSetAccountPictureResult** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* add_AccountPictureChanged)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIEventHandler_1_IInspectable* changeHandler,
        EventRegistrationToken* token);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* remove_AccountPictureChanged)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        EventRegistrationToken token);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetDisplayNameAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_HSTRING** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetFirstNameAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_HSTRING** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetLastNameAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_HSTRING** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetPrincipalNameAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_HSTRING** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetSessionInitiationProtocolUriAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_Windows__CFoundation__CUri** operation);
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    HRESULT (STDMETHODCALLTYPE* GetDomainNameAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics* This,
        __FIAsyncOperation_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_get_AccountPictureChangeEnabled(This, value) \
    ((This)->lpVtbl->get_AccountPictureChangeEnabled(This, value))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_get_NameAccessAllowed(This, value) \
    ((This)->lpVtbl->get_NameAccessAllowed(This, value))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetAccountPicture(This, kind, storageFile) \
    ((This)->lpVtbl->GetAccountPicture(This, kind, storageFile))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_SetAccountPictureAsync(This, image, operation) \
    ((This)->lpVtbl->SetAccountPictureAsync(This, image, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_SetAccountPicturesAsync(This, smallImage, largeImage, video, operation) \
    ((This)->lpVtbl->SetAccountPicturesAsync(This, smallImage, largeImage, video, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_SetAccountPictureFromStreamAsync(This, image, operation) \
    ((This)->lpVtbl->SetAccountPictureFromStreamAsync(This, image, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_SetAccountPicturesFromStreamsAsync(This, smallImage, largeImage, video, operation) \
    ((This)->lpVtbl->SetAccountPicturesFromStreamsAsync(This, smallImage, largeImage, video, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_add_AccountPictureChanged(This, changeHandler, token) \
    ((This)->lpVtbl->add_AccountPictureChanged(This, changeHandler, token))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_remove_AccountPictureChanged(This, token) \
    ((This)->lpVtbl->remove_AccountPictureChanged(This, token))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetDisplayNameAsync(This, operation) \
    ((This)->lpVtbl->GetDisplayNameAsync(This, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetFirstNameAsync(This, operation) \
    ((This)->lpVtbl->GetFirstNameAsync(This, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetLastNameAsync(This, operation) \
    ((This)->lpVtbl->GetLastNameAsync(This, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetPrincipalNameAsync(This, operation) \
    ((This)->lpVtbl->GetPrincipalNameAsync(This, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetSessionInitiationProtocolUriAsync(This, operation) \
    ((This)->lpVtbl->GetSessionInitiationProtocolUriAsync(This, operation))

#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
    DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_GetDomainNameAsync(This, operation) \
    ((This)->lpVtbl->GetDomainNameAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserProfilePersonalizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserProfilePersonalizationSettings[] = L"Windows.System.UserProfile.IUserProfilePersonalizationSettings";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TrySetLockScreenImageAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* imageFile,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* TrySetWallpaperImageAsync)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* imageFile,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_TrySetLockScreenImageAsync(This, imageFile, operation) \
    ((This)->lpVtbl->TrySetLockScreenImageAsync(This, imageFile, operation))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_TrySetWallpaperImageAsync(This, imageFile, operation) \
    ((This)->lpVtbl->TrySetWallpaperImageAsync(This, imageFile, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_UserProfile_IUserProfilePersonalizationSettingsStatics[] = L"Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettings** value);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CUserProfile_CIUserProfilePersonalizationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.AdvertisingManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IAdvertisingManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IAdvertisingManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManager_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AdvertisingManager[] = L"Windows.System.UserProfile.AdvertisingManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.AdvertisingManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IAdvertisingManagerForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AdvertisingManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AdvertisingManagerForUser[] = L"Windows.System.UserProfile.AdvertisingManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.UserProfile.AssignedAccessSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IAssignedAccessSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IAssignedAccessSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_AssignedAccessSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_AssignedAccessSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_AssignedAccessSettings[] = L"Windows.System.UserProfile.AssignedAccessSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.UserProfile.DiagnosticsSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IDiagnosticsSettingsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IDiagnosticsSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_DiagnosticsSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_DiagnosticsSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_DiagnosticsSettings[] = L"Windows.System.UserProfile.DiagnosticsSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.UserProfile.FirstSignInSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IFirstSignInSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IFirstSignInSettings ** Default Interface **
 *    Windows.Foundation.Collections.IMapView`2<String, Object>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_FirstSignInSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_FirstSignInSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_FirstSignInSettings[] = L"Windows.System.UserProfile.FirstSignInSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.GlobalizationPreferences
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.IGlobalizationPreferencesStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferences_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferences_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_GlobalizationPreferences[] = L"Windows.System.UserProfile.GlobalizationPreferences";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.GlobalizationPreferencesForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IGlobalizationPreferencesForUser ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferencesForUser_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_GlobalizationPreferencesForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_GlobalizationPreferencesForUser[] = L"Windows.System.UserProfile.GlobalizationPreferencesForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.System.UserProfile.LockScreen
 *
 * Introduced to Windows.System.UserProfile.UserProfileLockScreenContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.ILockScreenStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileLockScreenContract API contract
 *   Static Methods exist on the Windows.System.UserProfile.ILockScreenImageFeedStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileLockScreenContract API contract
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_LockScreen_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_LockScreen_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_LockScreen[] = L"Windows.System.UserProfile.LockScreen";
#endif
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILELOCKSCREENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.UserInformation
 *
 * Introduced to Windows.System.UserProfile.UserProfileContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IUserInformationStatics interface starting with version 1.0 of the Windows.System.UserProfile.UserProfileContract API contract
 *
 */
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_UserInformation_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_UserInformation_DEFINED
#if WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
DEPRECATED("Use User instead of UserInformation. For more info, see MSDN.")
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x20000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_UserInformation[] = L"Windows.System.UserProfile.UserInformation";
#endif
#endif // WINDOWS_SYSTEM_USERPROFILE_USERPROFILECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.UserProfile.UserProfilePersonalizationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.UserProfile.IUserProfilePersonalizationSettingsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.UserProfile.IUserProfilePersonalizationSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_UserProfile_UserProfilePersonalizationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_UserProfile_UserProfilePersonalizationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_UserProfile_UserProfilePersonalizationSettings[] = L"Windows.System.UserProfile.UserProfilePersonalizationSettings";
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
#endif // __windows2Esystem2Euserprofile_p_h__

#endif // __windows2Esystem2Euserprofile_h__
