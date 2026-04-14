
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
#ifndef __windows2Esystem2Eprofile_h__
#define __windows2Esystem2Eprofile_h__
#ifndef __windows2Esystem2Eprofile_p_h__
#define __windows2Esystem2Eprofile_p_h__


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

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION)
#define WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION)
#define WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION)
#define WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION)
#define WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IAnalyticsInfoStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics ABI::Windows::System::Profile::IAnalyticsInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IAnalyticsInfoStatics2;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2 ABI::Windows::System::Profile::IAnalyticsInfoStatics2

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IAnalyticsVersionInfo;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo ABI::Windows::System::Profile::IAnalyticsVersionInfo

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IAnalyticsVersionInfo2;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2 ABI::Windows::System::Profile::IAnalyticsVersionInfo2

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IAppApplicabilityStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics ABI::Windows::System::Profile::IAppApplicabilityStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IEducationSettingsStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics ABI::Windows::System::Profile::IEducationSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IHardwareIdentificationStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics ABI::Windows::System::Profile::IHardwareIdentificationStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IHardwareToken;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken ABI::Windows::System::Profile::IHardwareToken

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IKnownRetailInfoPropertiesStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics ABI::Windows::System::Profile::IKnownRetailInfoPropertiesStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IPlatformAutomaticAppSignInManagerStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics ABI::Windows::System::Profile::IPlatformAutomaticAppSignInManagerStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IPlatformDiagnosticsAndUsageDataSettingsStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics ABI::Windows::System::Profile::IPlatformDiagnosticsAndUsageDataSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IRetailInfoStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics ABI::Windows::System::Profile::IRetailInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISharedModeSettingsStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics ABI::Windows::System::Profile::ISharedModeSettingsStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISharedModeSettingsStatics2;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2 ABI::Windows::System::Profile::ISharedModeSettingsStatics2

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISmartAppControlPolicyStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics ABI::Windows::System::Profile::ISmartAppControlPolicyStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISystemIdentificationInfo;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo ABI::Windows::System::Profile::ISystemIdentificationInfo

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISystemIdentificationStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics ABI::Windows::System::Profile::ISystemIdentificationStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface ISystemSetupInfoStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics ABI::Windows::System::Profile::ISystemSetupInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IUnsupportedAppRequirement;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement ABI::Windows::System::Profile::IUnsupportedAppRequirement

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                interface IWindowsIntegrityPolicyStatics;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics ABI::Windows::System::Profile::IWindowsIntegrityPolicyStatics

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("817944b6-f046-5391-bb0b-4cc34d8040f3"))
IAsyncOperation<__FIMapView_2_HSTRING_HSTRING*> : IAsyncOperation_impl<__FIMapView_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IMapView`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIMapView_2_HSTRING_HSTRING*> __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_t;
#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("75e3182c-e6e1-589c-ab73-e8644bc285bf"))
IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIMapView_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IMapView`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIMapView_2_HSTRING_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_USE */



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


namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                class UnsupportedAppRequirement;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#define DEF___FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("10956160-6437-5430-9322-df0d6ab2ebe6"))
IIterator<ABI::Windows::System::Profile::UnsupportedAppRequirement*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Profile::UnsupportedAppRequirement*, ABI::Windows::System::Profile::IUnsupportedAppRequirement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.Profile.UnsupportedAppRequirement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::Profile::UnsupportedAppRequirement*> __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t;
#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#define DEF___FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bd9ded1d-0379-5143-a490-168b6b8413a3"))
IIterable<ABI::Windows::System::Profile::UnsupportedAppRequirement*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Profile::UnsupportedAppRequirement*, ABI::Windows::System::Profile::IUnsupportedAppRequirement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.Profile.UnsupportedAppRequirement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::Profile::UnsupportedAppRequirement*> __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t;
#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#define DEF___FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5b638c58-9d04-5d1a-92fb-860852c3e4d0"))
IVectorView<ABI::Windows::System::Profile::UnsupportedAppRequirement*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Profile::UnsupportedAppRequirement*, ABI::Windows::System::Profile::IUnsupportedAppRequirement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.Profile.UnsupportedAppRequirement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::Profile::UnsupportedAppRequirement*> __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t;
#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000


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
            namespace Profile {
                typedef enum PlatformAutomaticAppSignInPolicy : int PlatformAutomaticAppSignInPolicy;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                typedef enum PlatformDataCollectionLevel : int PlatformDataCollectionLevel;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                typedef enum SystemIdentificationSource : int SystemIdentificationSource;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                typedef enum SystemOutOfBoxExperienceState : int SystemOutOfBoxExperienceState;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                typedef enum UnsupportedAppRequirementReasons : unsigned int UnsupportedAppRequirementReasons;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                class AnalyticsVersionInfo;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                class HardwareToken;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                class SystemIdentificationInfo;
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Profile.PlatformAutomaticAppSignInPolicy
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                enum PlatformAutomaticAppSignInPolicy : int
                {
                    PlatformAutomaticAppSignInPolicy_Unknown = 0,
                    PlatformAutomaticAppSignInPolicy_PermissionRequired = 1,
                    PlatformAutomaticAppSignInPolicy_AlwaysAllowed = 2,
                };
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Profile.PlatformDataCollectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                enum PlatformDataCollectionLevel : int
                {
                    PlatformDataCollectionLevel_Security = 0,
                    PlatformDataCollectionLevel_Basic = 1,
                    PlatformDataCollectionLevel_Enhanced = 2,
                    PlatformDataCollectionLevel_Full = 3,
                };
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.System.Profile.SystemIdentificationSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                enum SystemIdentificationSource : int
                {
                    SystemIdentificationSource_None = 0,
                    SystemIdentificationSource_Tpm = 1,
                    SystemIdentificationSource_Uefi = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    SystemIdentificationSource_Registry = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                };
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.Profile.SystemOutOfBoxExperienceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                enum SystemOutOfBoxExperienceState : int
                {
                    SystemOutOfBoxExperienceState_NotStarted = 0,
                    SystemOutOfBoxExperienceState_InProgress = 1,
                    SystemOutOfBoxExperienceState_Completed = 2,
                };
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.System.Profile.UnsupportedAppRequirementReasons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                enum UnsupportedAppRequirementReasons : unsigned int
                {
                    UnsupportedAppRequirementReasons_Unknown = 0,
                    UnsupportedAppRequirementReasons_DeniedBySystem = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(UnsupportedAppRequirementReasons)
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsInfoStatics[] = L"Windows.System.Profile.IAnalyticsInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("1d5ee066-188d-5ba9-4387-acaeb0e7e305")
                IAnalyticsInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_VersionInfo(
                        ABI::Windows::System::Profile::IAnalyticsVersionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceForm(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAnalyticsInfoStatics = __uuidof(IAnalyticsInfoStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsInfoStatics2[] = L"Windows.System.Profile.IAnalyticsInfoStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("101704ea-a7f9-46d2-ab94-016865afdb25")
                IAnalyticsInfoStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetSystemPropertiesAsync(
                        __FIIterable_1_HSTRING* attributeNames,
                        __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAnalyticsInfoStatics2 = __uuidof(IAnalyticsInfoStatics2);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsVersionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsVersionInfo[] = L"Windows.System.Profile.IAnalyticsVersionInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("926130b8-9955-4c74-bdc1-7cd0decf9b03")
                IAnalyticsVersionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceFamily(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceFamilyVersion(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAnalyticsVersionInfo = __uuidof(IAnalyticsVersionInfo);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsVersionInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsVersionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsVersionInfo2[] = L"Windows.System.Profile.IAnalyticsVersionInfo2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("76e915b1-ff36-407c-9f57-160d3e540747")
                IAnalyticsVersionInfo2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProductName(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAnalyticsVersionInfo2 = __uuidof(IAnalyticsVersionInfo2);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.System.Profile.IAppApplicabilityStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AppApplicability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAppApplicabilityStatics[] = L"Windows.System.Profile.IAppApplicabilityStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("1664a082-0f38-5c99-83e4-48995970861c")
                IAppApplicabilityStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetUnsupportedAppRequirements(
                        __FIIterable_1_HSTRING* capabilities,
                        __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppApplicabilityStatics = __uuidof(IAppApplicabilityStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IEducationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.EducationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IEducationSettingsStatics[] = L"Windows.System.Profile.IEducationSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("fc53f0ef-4d3e-4e13-9b23-505f4d091e92")
                IEducationSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEducationEnvironment(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEducationSettingsStatics = __uuidof(IEducationSettingsStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Profile.IHardwareIdentificationStatics
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.HardwareIdentification
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IHardwareIdentificationStatics[] = L"Windows.System.Profile.IHardwareIdentificationStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("971260e0-f170-4a42-bd55-a900b212dae2")
                IHardwareIdentificationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetPackageSpecificToken(
                        ABI::Windows::Storage::Streams::IBuffer* nonce,
                        ABI::Windows::System::Profile::IHardwareToken** packageSpecificHardwareToken
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHardwareIdentificationStatics = __uuidof(IHardwareIdentificationStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IHardwareToken
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.HardwareToken
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IHardwareToken[] = L"Windows.System.Profile.IHardwareToken";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("28f6d4c0-fb12-40a4-8167-7f4e03d2724c")
                IHardwareToken : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Signature(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Certificate(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHardwareToken = __uuidof(IHardwareToken);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIHardwareToken;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IKnownRetailInfoPropertiesStatics
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.KnownRetailInfoProperties
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IKnownRetailInfoPropertiesStatics[] = L"Windows.System.Profile.IKnownRetailInfoPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("99571178-500f-487e-8e75-29e551728712")
                IKnownRetailInfoPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RetailAccessCode(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ManufacturerName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModelName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayModelName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Price(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsFeatured(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FormFactor(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScreenSize(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Weight(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BatteryLifeDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessorDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Memory(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StorageDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GraphicsDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrontCameraDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RearCameraDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasNfc(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasSdSlot(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasOpticalDrive(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOfficeInstalled(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WindowsEdition(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownRetailInfoPropertiesStatics = __uuidof(IKnownRetailInfoPropertiesStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.PlatformAutomaticAppSignInManager
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IPlatformAutomaticAppSignInManagerStatics[] = L"Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("1ac9afce-8dd5-5c2d-b420-767d1f3b7d03")
                IPlatformAutomaticAppSignInManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Policy(
                        ABI::Windows::System::Profile::PlatformAutomaticAppSignInPolicy* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlatformAutomaticAppSignInManagerStatics = __uuidof(IPlatformAutomaticAppSignInManagerStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IPlatformDiagnosticsAndUsageDataSettingsStatics[] = L"Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("b6e24c1b-7b1c-4b32-8c62-a66597ce723a")
                IPlatformDiagnosticsAndUsageDataSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CollectionLevel(
                        ABI::Windows::System::Profile::PlatformDataCollectionLevel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CollectionLevelChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CollectionLevelChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CanCollectDiagnostics(
                        ABI::Windows::System::Profile::PlatformDataCollectionLevel level,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPlatformDiagnosticsAndUsageDataSettingsStatics = __uuidof(IPlatformDiagnosticsAndUsageDataSettingsStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.System.Profile.IRetailInfoStatics
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.RetailInfo
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IRetailInfoStatics[] = L"Windows.System.Profile.IRetailInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("0712c6b8-8b92-4f2a-8499-031f1798d6ef")
                IRetailInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsDemoModeEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRetailInfoStatics = __uuidof(IRetailInfoStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.ISharedModeSettingsStatics
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SharedModeSettings
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISharedModeSettingsStatics[] = L"Windows.System.Profile.ISharedModeSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("893df40e-cad6-4d50-8c49-6fcfc03edb29")
                ISharedModeSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISharedModeSettingsStatics = __uuidof(ISharedModeSettingsStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.ISharedModeSettingsStatics2
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SharedModeSettings
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISharedModeSettingsStatics2[] = L"Windows.System.Profile.ISharedModeSettingsStatics2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("608988a4-ccf1-4ee8-a5e2-fd6a1d0cfac8")
                ISharedModeSettingsStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldAvoidLocalStorage(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISharedModeSettingsStatics2 = __uuidof(ISharedModeSettingsStatics2);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.System.Profile.ISmartAppControlPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SmartAppControlPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISmartAppControlPolicyStatics[] = L"Windows.System.Profile.ISmartAppControlPolicyStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("5ff8c75b-073e-5015-8d98-5ff224180a0b")
                ISmartAppControlPolicyStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmartAppControlPolicyStatics = __uuidof(ISmartAppControlPolicyStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.Profile.ISystemIdentificationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemIdentificationInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemIdentificationInfo[] = L"Windows.System.Profile.ISystemIdentificationInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("0c659e7d-c3c2-4d33-a2df-21bc41916eb3")
                ISystemIdentificationInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Source(
                        ABI::Windows::System::Profile::SystemIdentificationSource* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemIdentificationInfo = __uuidof(ISystemIdentificationInfo);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.Profile.ISystemIdentificationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemIdentification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemIdentificationStatics[] = L"Windows.System.Profile.ISystemIdentificationStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("5581f42a-d3df-4d93-a37d-c41a616c6d01")
                ISystemIdentificationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetSystemIdForPublisher(
                        ABI::Windows::System::Profile::ISystemIdentificationInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSystemIdForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::System::Profile::ISystemIdentificationInfo** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemIdentificationStatics = __uuidof(ISystemIdentificationStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.Profile.ISystemSetupInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemSetupInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemSetupInfoStatics[] = L"Windows.System.Profile.ISystemSetupInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("b8366a4b-fb6a-4571-be0a-9a0f67954123")
                ISystemSetupInfoStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OutOfBoxExperienceState(
                        ABI::Windows::System::Profile::SystemOutOfBoxExperienceState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_OutOfBoxExperienceStateChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_OutOfBoxExperienceStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISystemSetupInfoStatics = __uuidof(ISystemSetupInfoStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Profile.IUnsupportedAppRequirement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.UnsupportedAppRequirement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IUnsupportedAppRequirement[] = L"Windows.System.Profile.IUnsupportedAppRequirement";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("6182445c-894b-5cbc-8976-a98e0a9b998d")
                IUnsupportedAppRequirement : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Requirement(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reasons(
                        ABI::Windows::System::Profile::UnsupportedAppRequirementReasons* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnsupportedAppRequirement = __uuidof(IUnsupportedAppRequirement);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IWindowsIntegrityPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.WindowsIntegrityPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IWindowsIntegrityPolicyStatics[] = L"Windows.System.Profile.IWindowsIntegrityPolicyStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Profile {
                MIDL_INTERFACE("7d1d81db-8d63-4789-9ea5-ddcf65a94f3c")
                IWindowsIntegrityPolicyStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabledForTrial(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanDisable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDisableSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_PolicyChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_PolicyChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsIntegrityPolicyStatics = __uuidof(IWindowsIntegrityPolicyStatics);
            } /* Profile */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Profile.AnalyticsInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IAnalyticsInfoStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Profile.IAnalyticsInfoStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_AnalyticsInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AnalyticsInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AnalyticsInfo[] = L"Windows.System.Profile.AnalyticsInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.AnalyticsVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IAnalyticsVersionInfo ** Default Interface **
 *    Windows.System.Profile.IAnalyticsVersionInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_AnalyticsVersionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AnalyticsVersionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AnalyticsVersionInfo[] = L"Windows.System.Profile.AnalyticsVersionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.AppApplicability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IAppApplicabilityStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_System_Profile_AppApplicability_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AppApplicability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AppApplicability[] = L"Windows.System.Profile.AppApplicability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.System.Profile.EducationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IEducationSettingsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Profile_EducationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_EducationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_EducationSettings[] = L"Windows.System.Profile.EducationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Profile.HardwareIdentification
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IHardwareIdentificationStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileHardwareTokenContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_HardwareIdentification_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_HardwareIdentification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_HardwareIdentification[] = L"Windows.System.Profile.HardwareIdentification";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.HardwareToken
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IHardwareToken ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_HardwareToken_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_HardwareToken_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_HardwareToken[] = L"Windows.System.Profile.HardwareToken";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.KnownRetailInfoProperties
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IKnownRetailInfoPropertiesStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileRetailInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_KnownRetailInfoProperties_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_KnownRetailInfoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_KnownRetailInfoProperties[] = L"Windows.System.Profile.KnownRetailInfoProperties";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.PlatformAutomaticAppSignInManager
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics interface starting with version 1.0 of the Windows.System.Profile.PlatformAutomaticAppSignInContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_PlatformAutomaticAppSignInManager_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_PlatformAutomaticAppSignInManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_PlatformAutomaticAppSignInManager[] = L"Windows.System.Profile.PlatformAutomaticAppSignInManager";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings[] = L"Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.System.Profile.RetailInfo
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IRetailInfoStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileRetailInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_RetailInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_RetailInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_RetailInfo[] = L"Windows.System.Profile.RetailInfo";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.SharedModeSettings
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISharedModeSettingsStatics2 interface starting with version 2.0 of the Windows.System.Profile.ProfileSharedModeContract API contract
 *   Static Methods exist on the Windows.System.Profile.ISharedModeSettingsStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileSharedModeContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_SharedModeSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SharedModeSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SharedModeSettings[] = L"Windows.System.Profile.SharedModeSettings";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.SmartAppControlPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISmartAppControlPolicyStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_Profile_SmartAppControlPolicy_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SmartAppControlPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SmartAppControlPolicy[] = L"Windows.System.Profile.SmartAppControlPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.Profile.SystemIdentification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISystemIdentificationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemIdentification_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemIdentification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemIdentification[] = L"Windows.System.Profile.SystemIdentification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.Profile.SystemIdentificationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.ISystemIdentificationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemIdentificationInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemIdentificationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemIdentificationInfo[] = L"Windows.System.Profile.SystemIdentificationInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.Profile.SystemSetupInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISystemSetupInfoStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemSetupInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemSetupInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemSetupInfo[] = L"Windows.System.Profile.SystemSetupInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Profile.UnsupportedAppRequirement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IUnsupportedAppRequirement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_System_Profile_UnsupportedAppRequirement_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_UnsupportedAppRequirement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_UnsupportedAppRequirement[] = L"Windows.System.Profile.UnsupportedAppRequirement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.System.Profile.WindowsIntegrityPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IWindowsIntegrityPolicyStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Profile_WindowsIntegrityPolicy_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_WindowsIntegrityPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_WindowsIntegrityPolicy[] = L"Windows.System.Profile.WindowsIntegrityPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2 __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2 __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2 __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics;

#endif // ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING* This,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

typedef struct __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl;

interface __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

typedef struct __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        __FIIterator_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl;

interface __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement;

typedef struct __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl;

interface __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CProfile_CPlatformAutomaticAppSignInPolicy __x_ABI_CWindows_CSystem_CProfile_CPlatformAutomaticAppSignInPolicy;

typedef enum __x_ABI_CWindows_CSystem_CProfile_CPlatformDataCollectionLevel __x_ABI_CWindows_CSystem_CProfile_CPlatformDataCollectionLevel;

typedef enum __x_ABI_CWindows_CSystem_CProfile_CSystemIdentificationSource __x_ABI_CWindows_CSystem_CProfile_CSystemIdentificationSource;

typedef enum __x_ABI_CWindows_CSystem_CProfile_CSystemOutOfBoxExperienceState __x_ABI_CWindows_CSystem_CProfile_CSystemOutOfBoxExperienceState;

typedef enum __x_ABI_CWindows_CSystem_CProfile_CUnsupportedAppRequirementReasons __x_ABI_CWindows_CSystem_CProfile_CUnsupportedAppRequirementReasons;

/*
 *
 * Struct Windows.System.Profile.PlatformAutomaticAppSignInPolicy
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CSystem_CProfile_CPlatformAutomaticAppSignInPolicy
{
    PlatformAutomaticAppSignInPolicy_Unknown = 0,
    PlatformAutomaticAppSignInPolicy_PermissionRequired = 1,
    PlatformAutomaticAppSignInPolicy_AlwaysAllowed = 2,
};
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.System.Profile.PlatformDataCollectionLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CSystem_CProfile_CPlatformDataCollectionLevel
{
    PlatformDataCollectionLevel_Security = 0,
    PlatformDataCollectionLevel_Basic = 1,
    PlatformDataCollectionLevel_Enhanced = 2,
    PlatformDataCollectionLevel_Full = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.System.Profile.SystemIdentificationSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CSystem_CProfile_CSystemIdentificationSource
{
    SystemIdentificationSource_None = 0,
    SystemIdentificationSource_Tpm = 1,
    SystemIdentificationSource_Uefi = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    SystemIdentificationSource_Registry = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.System.Profile.SystemOutOfBoxExperienceState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CSystem_CProfile_CSystemOutOfBoxExperienceState
{
    SystemOutOfBoxExperienceState_NotStarted = 0,
    SystemOutOfBoxExperienceState_InProgress = 1,
    SystemOutOfBoxExperienceState_Completed = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.System.Profile.UnsupportedAppRequirementReasons
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
enum __x_ABI_CWindows_CSystem_CProfile_CUnsupportedAppRequirementReasons
{
    UnsupportedAppRequirementReasons_Unknown = 0,
    UnsupportedAppRequirementReasons_DeniedBySystem = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsInfoStatics[] = L"Windows.System.Profile.IAnalyticsInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VersionInfo)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceForm)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_get_VersionInfo(This, value) \
    ((This)->lpVtbl->get_VersionInfo(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_get_DeviceForm(This, value) \
    ((This)->lpVtbl->get_DeviceForm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsInfoStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsInfoStatics2[] = L"Windows.System.Profile.IAnalyticsInfoStatics2";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSystemPropertiesAsync)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2* This,
        __FIIterable_1_HSTRING* attributeNames,
        __FIAsyncOperation_1___FIMapView_2_HSTRING_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_GetSystemPropertiesAsync(This, attributeNames, operation) \
    ((This)->lpVtbl->GetSystemPropertiesAsync(This, attributeNames, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsInfoStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsVersionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsVersionInfo[] = L"Windows.System.Profile.IAnalyticsVersionInfo";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceFamily)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceFamilyVersion)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfoVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_get_DeviceFamily(This, value) \
    ((This)->lpVtbl->get_DeviceFamily(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_get_DeviceFamilyVersion(This, value) \
    ((This)->lpVtbl->get_DeviceFamilyVersion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IAnalyticsVersionInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AnalyticsVersionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAnalyticsVersionInfo2[] = L"Windows.System.Profile.IAnalyticsVersionInfo2";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductName)(__x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2Vtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_get_ProductName(This, value) \
    ((This)->lpVtbl->get_ProductName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAnalyticsVersionInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.System.Profile.IAppApplicabilityStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.AppApplicability
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IAppApplicabilityStatics[] = L"Windows.System.Profile.IAppApplicabilityStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetUnsupportedAppRequirements)(__x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics* This,
        __FIIterable_1_HSTRING* capabilities,
        __FIVectorView_1_Windows__CSystem__CProfile__CUnsupportedAppRequirement** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_GetUnsupportedAppRequirements(This, capabilities, result) \
    ((This)->lpVtbl->GetUnsupportedAppRequirements(This, capabilities, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIAppApplicabilityStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IEducationSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.EducationSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IEducationSettingsStatics[] = L"Windows.System.Profile.IEducationSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEducationEnvironment)(__x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_get_IsEducationEnvironment(This, value) \
    ((This)->lpVtbl->get_IsEducationEnvironment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIEducationSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.System.Profile.IHardwareIdentificationStatics
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.HardwareIdentification
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IHardwareIdentificationStatics[] = L"Windows.System.Profile.IHardwareIdentificationStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPackageSpecificToken)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* nonce,
        __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken** packageSpecificHardwareToken);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_GetPackageSpecificToken(This, nonce, packageSpecificHardwareToken) \
    ((This)->lpVtbl->GetPackageSpecificToken(This, nonce, packageSpecificHardwareToken))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareIdentificationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IHardwareToken
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.HardwareToken
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IHardwareToken[] = L"Windows.System.Profile.IHardwareToken";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIHardwareTokenVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Signature)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Certificate)(__x_ABI_CWindows_CSystem_CProfile_CIHardwareToken* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIHardwareTokenVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIHardwareTokenVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_get_Signature(This, value) \
    ((This)->lpVtbl->get_Signature(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_get_Certificate(This, value) \
    ((This)->lpVtbl->get_Certificate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIHardwareToken;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIHardwareToken_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IKnownRetailInfoPropertiesStatics
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.KnownRetailInfoProperties
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IKnownRetailInfoPropertiesStatics[] = L"Windows.System.Profile.IKnownRetailInfoPropertiesStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RetailAccessCode)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ManufacturerName)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ModelName)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayModelName)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Price)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsFeatured)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FormFactor)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ScreenSize)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Weight)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BatteryLifeDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProcessorDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Memory)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StorageDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_GraphicsDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FrontCameraDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RearCameraDescription)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HasNfc)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HasSdSlot)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HasOpticalDrive)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOfficeInstalled)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_WindowsEdition)(__x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_RetailAccessCode(This, value) \
    ((This)->lpVtbl->get_RetailAccessCode(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_ManufacturerName(This, value) \
    ((This)->lpVtbl->get_ManufacturerName(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_ModelName(This, value) \
    ((This)->lpVtbl->get_ModelName(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_DisplayModelName(This, value) \
    ((This)->lpVtbl->get_DisplayModelName(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_Price(This, value) \
    ((This)->lpVtbl->get_Price(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_IsFeatured(This, value) \
    ((This)->lpVtbl->get_IsFeatured(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_FormFactor(This, value) \
    ((This)->lpVtbl->get_FormFactor(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_ScreenSize(This, value) \
    ((This)->lpVtbl->get_ScreenSize(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_Weight(This, value) \
    ((This)->lpVtbl->get_Weight(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_DisplayDescription(This, value) \
    ((This)->lpVtbl->get_DisplayDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_BatteryLifeDescription(This, value) \
    ((This)->lpVtbl->get_BatteryLifeDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_ProcessorDescription(This, value) \
    ((This)->lpVtbl->get_ProcessorDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_Memory(This, value) \
    ((This)->lpVtbl->get_Memory(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_StorageDescription(This, value) \
    ((This)->lpVtbl->get_StorageDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_GraphicsDescription(This, value) \
    ((This)->lpVtbl->get_GraphicsDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_FrontCameraDescription(This, value) \
    ((This)->lpVtbl->get_FrontCameraDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_RearCameraDescription(This, value) \
    ((This)->lpVtbl->get_RearCameraDescription(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_HasNfc(This, value) \
    ((This)->lpVtbl->get_HasNfc(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_HasSdSlot(This, value) \
    ((This)->lpVtbl->get_HasSdSlot(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_HasOpticalDrive(This, value) \
    ((This)->lpVtbl->get_HasOpticalDrive(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_IsOfficeInstalled(This, value) \
    ((This)->lpVtbl->get_IsOfficeInstalled(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_get_WindowsEdition(This, value) \
    ((This)->lpVtbl->get_WindowsEdition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIKnownRetailInfoPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.PlatformAutomaticAppSignInManager
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IPlatformAutomaticAppSignInManagerStatics[] = L"Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Policy)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CPlatformAutomaticAppSignInPolicy* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_get_Policy(This, value) \
    ((This)->lpVtbl->get_Policy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformAutomaticAppSignInManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IPlatformDiagnosticsAndUsageDataSettingsStatics[] = L"Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionLevel)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CPlatformDataCollectionLevel* value);
    HRESULT (STDMETHODCALLTYPE* add_CollectionLevelChanged)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CollectionLevelChanged)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CanCollectDiagnostics)(__x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CPlatformDataCollectionLevel level,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_get_CollectionLevel(This, value) \
    ((This)->lpVtbl->get_CollectionLevel(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_add_CollectionLevelChanged(This, handler, token) \
    ((This)->lpVtbl->add_CollectionLevelChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_remove_CollectionLevelChanged(This, token) \
    ((This)->lpVtbl->remove_CollectionLevelChanged(This, token))

#define __x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_CanCollectDiagnostics(This, level, result) \
    ((This)->lpVtbl->CanCollectDiagnostics(This, level, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIPlatformDiagnosticsAndUsageDataSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.System.Profile.IRetailInfoStatics
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.RetailInfo
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IRetailInfoStatics[] = L"Windows.System.Profile.IRetailInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDemoModeEnabled)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_get_IsDemoModeEnabled(This, value) \
    ((This)->lpVtbl->get_IsDemoModeEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIRetailInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.ISharedModeSettingsStatics
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SharedModeSettings
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISharedModeSettingsStatics[] = L"Windows.System.Profile.ISharedModeSettingsStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.System.Profile.ISharedModeSettingsStatics2
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SharedModeSettings
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISharedModeSettingsStatics2[] = L"Windows.System.Profile.ISharedModeSettingsStatics2";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShouldAvoidLocalStorage)(__x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2Vtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_get_ShouldAvoidLocalStorage(This, value) \
    ((This)->lpVtbl->get_ShouldAvoidLocalStorage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISharedModeSettingsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.System.Profile.ISmartAppControlPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SmartAppControlPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISmartAppControlPolicyStatics[] = L"Windows.System.Profile.ISmartAppControlPolicyStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISmartAppControlPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.Profile.ISystemIdentificationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemIdentificationInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemIdentificationInfo[] = L"Windows.System.Profile.ISystemIdentificationInfo";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Source)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CSystemIdentificationSource* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfoVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_get_Source(This, value) \
    ((This)->lpVtbl->get_Source(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.Profile.ISystemIdentificationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemIdentification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemIdentificationStatics[] = L"Windows.System.Profile.ISystemIdentificationStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSystemIdForPublisher)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetSystemIdForUser)(__x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_GetSystemIdForPublisher(This, result) \
    ((This)->lpVtbl->GetSystemIdForPublisher(This, result))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_GetSystemIdForUser(This, user, result) \
    ((This)->lpVtbl->GetSystemIdForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemIdentificationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.System.Profile.ISystemSetupInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.SystemSetupInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_ISystemSetupInfoStatics[] = L"Windows.System.Profile.ISystemSetupInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OutOfBoxExperienceState)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CSystemOutOfBoxExperienceState* value);
    HRESULT (STDMETHODCALLTYPE* add_OutOfBoxExperienceStateChanged)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OutOfBoxExperienceStateChanged)(__x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_get_OutOfBoxExperienceState(This, value) \
    ((This)->lpVtbl->get_OutOfBoxExperienceState(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_add_OutOfBoxExperienceStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_OutOfBoxExperienceStateChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_remove_OutOfBoxExperienceStateChanged(This, token) \
    ((This)->lpVtbl->remove_OutOfBoxExperienceStateChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CISystemSetupInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Profile.IUnsupportedAppRequirement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.UnsupportedAppRequirement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IUnsupportedAppRequirement[] = L"Windows.System.Profile.IUnsupportedAppRequirement";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Requirement)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Reasons)(__x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement* This,
        enum __x_ABI_CWindows_CSystem_CProfile_CUnsupportedAppRequirementReasons* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirementVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_get_Requirement(This, value) \
    ((This)->lpVtbl->get_Requirement(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_get_Reasons(This, value) \
    ((This)->lpVtbl->get_Reasons(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIUnsupportedAppRequirement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.System.Profile.IWindowsIntegrityPolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Profile.WindowsIntegrityPolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Profile_IWindowsIntegrityPolicyStatics[] = L"Windows.System.Profile.IWindowsIntegrityPolicyStatics";
typedef struct __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabledForTrial)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CanDisable)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDisableSupported)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_PolicyChanged)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PolicyChanged)(__x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_get_IsEnabledForTrial(This, value) \
    ((This)->lpVtbl->get_IsEnabledForTrial(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_get_CanDisable(This, value) \
    ((This)->lpVtbl->get_CanDisable(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_get_IsDisableSupported(This, value) \
    ((This)->lpVtbl->get_IsDisableSupported(This, value))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_add_PolicyChanged(This, handler, token) \
    ((This)->lpVtbl->add_PolicyChanged(This, handler, token))

#define __x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_remove_PolicyChanged(This, token) \
    ((This)->lpVtbl->remove_PolicyChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CProfile_CIWindowsIntegrityPolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Profile.AnalyticsInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IAnalyticsInfoStatics2 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.System.Profile.IAnalyticsInfoStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_AnalyticsInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AnalyticsInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AnalyticsInfo[] = L"Windows.System.Profile.AnalyticsInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.AnalyticsVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IAnalyticsVersionInfo ** Default Interface **
 *    Windows.System.Profile.IAnalyticsVersionInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_AnalyticsVersionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AnalyticsVersionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AnalyticsVersionInfo[] = L"Windows.System.Profile.AnalyticsVersionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.AppApplicability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IAppApplicabilityStatics interface starting with version 8.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_System_Profile_AppApplicability_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_AppApplicability_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_AppApplicability[] = L"Windows.System.Profile.AppApplicability";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.System.Profile.EducationSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IEducationSettingsStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_System_Profile_EducationSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_EducationSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_EducationSettings[] = L"Windows.System.Profile.EducationSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.System.Profile.HardwareIdentification
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IHardwareIdentificationStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileHardwareTokenContract API contract
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_HardwareIdentification_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_HardwareIdentification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_HardwareIdentification[] = L"Windows.System.Profile.HardwareIdentification";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.HardwareToken
 *
 * Introduced to Windows.System.Profile.ProfileHardwareTokenContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IHardwareToken ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_HardwareToken_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_HardwareToken_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_HardwareToken[] = L"Windows.System.Profile.HardwareToken";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILEHARDWARETOKENCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.KnownRetailInfoProperties
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IKnownRetailInfoPropertiesStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileRetailInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_KnownRetailInfoProperties_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_KnownRetailInfoProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_KnownRetailInfoProperties[] = L"Windows.System.Profile.KnownRetailInfoProperties";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.PlatformAutomaticAppSignInManager
 *
 * Introduced to Windows.System.Profile.PlatformAutomaticAppSignInContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IPlatformAutomaticAppSignInManagerStatics interface starting with version 1.0 of the Windows.System.Profile.PlatformAutomaticAppSignInContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_PlatformAutomaticAppSignInManager_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_PlatformAutomaticAppSignInManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_PlatformAutomaticAppSignInManager[] = L"Windows.System.Profile.PlatformAutomaticAppSignInManager";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PLATFORMAUTOMATICAPPSIGNINCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IPlatformDiagnosticsAndUsageDataSettingsStatics interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_PlatformDiagnosticsAndUsageDataSettings[] = L"Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.System.Profile.RetailInfo
 *
 * Introduced to Windows.System.Profile.ProfileRetailInfoContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IRetailInfoStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileRetailInfoContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_RetailInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_RetailInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_RetailInfo[] = L"Windows.System.Profile.RetailInfo";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILERETAILINFOCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.SharedModeSettings
 *
 * Introduced to Windows.System.Profile.ProfileSharedModeContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISharedModeSettingsStatics2 interface starting with version 2.0 of the Windows.System.Profile.ProfileSharedModeContract API contract
 *   Static Methods exist on the Windows.System.Profile.ISharedModeSettingsStatics interface starting with version 1.0 of the Windows.System.Profile.ProfileSharedModeContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_System_Profile_SharedModeSettings_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SharedModeSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SharedModeSettings[] = L"Windows.System.Profile.SharedModeSettings";
#endif
#endif // WINDOWS_SYSTEM_PROFILE_PROFILESHAREDMODECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.System.Profile.SmartAppControlPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISmartAppControlPolicyStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_Profile_SmartAppControlPolicy_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SmartAppControlPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SmartAppControlPolicy[] = L"Windows.System.Profile.SmartAppControlPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.Profile.SystemIdentification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISystemIdentificationStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemIdentification_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemIdentification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemIdentification[] = L"Windows.System.Profile.SystemIdentification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.Profile.SystemIdentificationInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.ISystemIdentificationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemIdentificationInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemIdentificationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemIdentificationInfo[] = L"Windows.System.Profile.SystemIdentificationInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.System.Profile.SystemSetupInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.ISystemSetupInfoStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Profile_SystemSetupInfo_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_SystemSetupInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_SystemSetupInfo[] = L"Windows.System.Profile.SystemSetupInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Profile.UnsupportedAppRequirement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Profile.IUnsupportedAppRequirement ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_System_Profile_UnsupportedAppRequirement_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_UnsupportedAppRequirement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_UnsupportedAppRequirement[] = L"Windows.System.Profile.UnsupportedAppRequirement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.System.Profile.WindowsIntegrityPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Profile.IWindowsIntegrityPolicyStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Profile_WindowsIntegrityPolicy_DEFINED
#define RUNTIMECLASS_Windows_System_Profile_WindowsIntegrityPolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Profile_WindowsIntegrityPolicy[] = L"Windows.System.Profile.WindowsIntegrityPolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eprofile_p_h__

#endif // __windows2Esystem2Eprofile_h__
