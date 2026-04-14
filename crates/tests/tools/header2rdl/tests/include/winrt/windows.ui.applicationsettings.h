
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
#ifndef __windows2Eui2Eapplicationsettings_h__
#define __windows2Eui2Eapplicationsettings_h__
#ifndef __windows2Eui2Eapplicationsettings_p_h__
#define __windows2Eui2Eapplicationsettings_p_h__


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

#if !defined(WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION)
#define WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Security.Credentials.h"
#include "Windows.System.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ICredentialCommandCredentialDeletedHandler;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler ABI::Windows::UI::ApplicationSettings::ICredentialCommandCredentialDeletedHandler

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountCommandInvokedHandler;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler ABI::Windows::UI::ApplicationSettings::IWebAccountCommandInvokedHandler

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountProviderCommandInvokedHandler;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommandInvokedHandler

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPane;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPane

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneCommandsRequestedEventArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneCommandsRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneCommandsRequestedEventArgs2;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2 ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneCommandsRequestedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneEventDeferral;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneEventDeferral

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneStatics;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneStatics

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneStatics2;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2 ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneStatics2

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IAccountsSettingsPaneStatics3;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3 ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneStatics3

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ICredentialCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand ABI::Windows::UI::ApplicationSettings::ICredentialCommand

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ICredentialCommandFactory;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory ABI::Windows::UI::ApplicationSettings::ICredentialCommandFactory

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsCommandFactory;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory ABI::Windows::UI::ApplicationSettings::ISettingsCommandFactory

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsCommandStatics;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics ABI::Windows::UI::ApplicationSettings::ISettingsCommandStatics

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsPane;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane ABI::Windows::UI::ApplicationSettings::ISettingsPane

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsPaneCommandsRequest;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest ABI::Windows::UI::ApplicationSettings::ISettingsPaneCommandsRequest

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsPaneCommandsRequestedEventArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs ABI::Windows::UI::ApplicationSettings::ISettingsPaneCommandsRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface ISettingsPaneStatics;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics ABI::Windows::UI::ApplicationSettings::ISettingsPaneStatics

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand ABI::Windows::UI::ApplicationSettings::IWebAccountCommand

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountCommandFactory;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory ABI::Windows::UI::ApplicationSettings::IWebAccountCommandFactory

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountInvokedArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs ABI::Windows::UI::ApplicationSettings::IWebAccountInvokedArgs

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountProviderCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                interface IWebAccountProviderCommandFactory;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommandFactory

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class CredentialCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#define DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9f1177f1-85bb-5cd0-9b08-a0b47a764c75"))
IIterator<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::CredentialCommand*, ABI::Windows::UI::ApplicationSettings::ICredentialCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.ApplicationSettings.CredentialCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t;
#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#define DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("883ed18d-4dbb-58f2-8fd2-e4b018509553"))
IIterable<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::CredentialCommand*, ABI::Windows::UI::ApplicationSettings::ICredentialCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.ApplicationSettings.CredentialCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t;
#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class SettingsCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommand;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommand ABI::Windows::UI::Popups::IUICommand

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#define DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f071c24-4a58-5a00-a294-c7162e98c2a0"))
IIterator<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsCommand*, ABI::Windows::UI::Popups::IUICommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.ApplicationSettings.SettingsCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t;
#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#define DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6308e7e8-cb85-5339-a3e9-9a7500d19c68"))
IIterable<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsCommand*, ABI::Windows::UI::Popups::IUICommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.ApplicationSettings.SettingsCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t;
#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class WebAccountCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#define DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8cbb62b6-bd9c-5486-9d14-9cc4627b32d4"))
IIterator<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.ApplicationSettings.WebAccountCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t;
#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#define DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bd0d999c-b2ba-51b2-bcc0-d4a5cd821555"))
IIterable<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.ApplicationSettings.WebAccountCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t;
#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class WebAccountProviderCommand;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#define DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("82d7cd74-8e33-5f06-92fc-915138aacbde"))
IIterator<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.ApplicationSettings.WebAccountProviderCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t;
#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#define DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("15165367-2e93-59a6-b5c7-16d3b58fd2e7"))
IIterable<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.ApplicationSettings.WebAccountProviderCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t;
#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("95cc1bba-c279-5ee5-a524-78012b7fe17e"))
IVectorView<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::CredentialCommand*, ABI::Windows::UI::ApplicationSettings::ICredentialCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.ApplicationSettings.CredentialCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t;
#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("67b64d17-4245-5d7c-bfb4-6b68dd525877"))
IVectorView<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsCommand*, ABI::Windows::UI::Popups::IUICommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.ApplicationSettings.SettingsCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t;
#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("207eaa3e-5ec9-5bd4-a1d2-73179a8128a8"))
IVectorView<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.ApplicationSettings.WebAccountCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t;
#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#define DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b2d19260-1827-5d88-b948-9688cfcd63ae"))
IVectorView<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.ApplicationSettings.WebAccountProviderCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t;
#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#define DEF___FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6af1cb5-f60e-5b08-b312-2eb51135cfc6"))
IVector<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::CredentialCommand*, ABI::Windows::UI::ApplicationSettings::ICredentialCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.ApplicationSettings.CredentialCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::ApplicationSettings::CredentialCommand*> __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t;
#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#define DEF___FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("10bd9cdd-3767-5e96-9022-f00f9cbd6241"))
IVector<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsCommand*, ABI::Windows::UI::Popups::IUICommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.ApplicationSettings.SettingsCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::ApplicationSettings::SettingsCommand*> __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t;
#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#define DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64e864c8-7fef-5df5-a624-50b577f48554"))
IVector<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.ApplicationSettings.WebAccountCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::ApplicationSettings::WebAccountCommand*> __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t;
#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#define DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d376abf3-f0c1-5233-9f42-de531884963e"))
IVector<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*, ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.ApplicationSettings.WebAccountProviderCommand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::ApplicationSettings::WebAccountProviderCommand*> __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t;
#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class AccountsSettingsPane;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class AccountsSettingsPaneCommandsRequestedEventArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("69b8847e-7d72-5a15-bc1c-4ca39c93b162"))
ITypedEventHandler<ABI::Windows::UI::ApplicationSettings::AccountsSettingsPane*, ABI::Windows::UI::ApplicationSettings::AccountsSettingsPaneCommandsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::AccountsSettingsPane*, ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::AccountsSettingsPaneCommandsRequestedEventArgs*, ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneCommandsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ApplicationSettings.AccountsSettingsPane, Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ApplicationSettings::AccountsSettingsPane*, ABI::Windows::UI::ApplicationSettings::AccountsSettingsPaneCommandsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class SettingsPane;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class SettingsPaneCommandsRequestedEventArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f39a56a2-7db1-5c48-9e13-7dc485a4a99e"))
ITypedEventHandler<ABI::Windows::UI::ApplicationSettings::SettingsPane*, ABI::Windows::UI::ApplicationSettings::SettingsPaneCommandsRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsPane*, ABI::Windows::UI::ApplicationSettings::ISettingsPane*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::ApplicationSettings::SettingsPaneCommandsRequestedEventArgs*, ABI::Windows::UI::ApplicationSettings::ISettingsPaneCommandsRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.ApplicationSettings.SettingsPane, Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::ApplicationSettings::SettingsPane*, ABI::Windows::UI::ApplicationSettings::SettingsPaneCommandsRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_USE */

#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

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
        namespace Security {
            namespace Credentials {
                class PasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IPasswordCredential;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential ABI::Windows::Security::Credentials::IPasswordCredential

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount ABI::Windows::Security::Credentials::IWebAccount

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccountProvider;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccountProvider;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider ABI::Windows::Security::Credentials::IWebAccountProvider

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                interface IUICommandInvokedHandler;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler ABI::Windows::UI::Popups::IUICommandInvokedHandler

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                typedef enum SettingsEdgeLocation : int SettingsEdgeLocation;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                typedef enum SupportedWebAccountActions : unsigned int SupportedWebAccountActions;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                typedef enum WebAccountAction : int WebAccountAction;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class AccountsSettingsPaneEventDeferral;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class SettingsPaneCommandsRequest;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                class WebAccountInvokedArgs;
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.ApplicationSettings.SettingsEdgeLocation
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                enum
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                DEPRECATED("SettingsEdgeLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                SettingsEdgeLocation : int
                {
                    SettingsEdgeLocation_Right = 0,
                    SettingsEdgeLocation_Left = 1,
                };
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ApplicationSettings.SupportedWebAccountActions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                enum SupportedWebAccountActions : unsigned int
                {
                    SupportedWebAccountActions_None = 0,
                    SupportedWebAccountActions_Reconnect = 0x1,
                    SupportedWebAccountActions_Remove = 0x2,
                    SupportedWebAccountActions_ViewDetails = 0x4,
                    SupportedWebAccountActions_Manage = 0x8,
                    SupportedWebAccountActions_More = 0x10,
                };

                DEFINE_ENUM_FLAG_OPERATORS(SupportedWebAccountActions)
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ApplicationSettings.WebAccountAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                enum WebAccountAction : int
                {
                    WebAccountAction_Reconnect = 0,
                    WebAccountAction_Remove = 1,
                    WebAccountAction_ViewDetails = 2,
                    WebAccountAction_Manage = 3,
                    WebAccountAction_More = 4,
                };
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.CredentialCommandCredentialDeletedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("61c0e185-0977-4678-b4e2-98727afbeed9")
                ICredentialCommandCredentialDeletedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::UI::ApplicationSettings::ICredentialCommand* command
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICredentialCommandCredentialDeletedHandler = __uuidof(ICredentialCommandCredentialDeletedHandler);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.WebAccountCommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("1ee6e459-1705-4a9a-b599-a0c3d6921973")
                IWebAccountCommandInvokedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::UI::ApplicationSettings::IWebAccountCommand* command,
                        ABI::Windows::UI::ApplicationSettings::IWebAccountInvokedArgs* args
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountCommandInvokedHandler = __uuidof(IWebAccountCommandInvokedHandler);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.WebAccountProviderCommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("b7de5527-4c8f-42dd-84da-5ec493abdb9a")
                IWebAccountProviderCommandInvokedHandler : public IUnknown
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand* command
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProviderCommandInvokedHandler = __uuidof(IWebAccountProviderCommandInvokedHandler);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPane[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPane";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("81ea942c-4f09-4406-a538-838d9b14b7e6")
                IAccountsSettingsPane : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AccountCommandsRequested(
                        __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AccountCommandsRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPane = __uuidof(IAccountsSettingsPane);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("3b68c099-db19-45d0-9abf-95d3773c9330")
                IAccountsSettingsPaneCommandsRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebAccountProviderCommands(
                        __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WebAccountCommands(
                        __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CredentialCommands(
                        __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Commands(
                        __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HeaderText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPaneEventDeferral** deferral
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneCommandsRequestedEventArgs = __uuidof(IAccountsSettingsPaneCommandsRequestedEventArgs);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneCommandsRequestedEventArgs2[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("362f7bad-4e37-4967-8c40-e78ee7a1e5bb")
                IAccountsSettingsPaneCommandsRequestedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneCommandsRequestedEventArgs2 = __uuidof(IAccountsSettingsPaneCommandsRequestedEventArgs2);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneEventDeferral[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("cbf25d3f-e5ba-40ef-93da-65e096e5fb04")
                IAccountsSettingsPaneEventDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneEventDeferral = __uuidof(IAccountsSettingsPaneEventDeferral);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("561f8b60-b0ec-4150-a8dc-208ee44b068a")
                IAccountsSettingsPaneStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ApplicationSettings::IAccountsSettingsPane** current
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Show(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneStatics = __uuidof(IAccountsSettingsPaneStatics);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics2[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("d21df7c2-ce0d-484f-b8e8-e823c215765e")
                IAccountsSettingsPaneStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowManageAccountsAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAccountAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneStatics2 = __uuidof(IAccountsSettingsPaneStatics2);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics3[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("08410458-a2ba-4c6f-b4ac-48f514331216")
                IAccountsSettingsPaneStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowManageAccountsForUserAsync(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAccountForUserAsync(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAccountsSettingsPaneStatics3 = __uuidof(IAccountsSettingsPaneStatics3);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ICredentialCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.CredentialCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ICredentialCommand[] = L"Windows.UI.ApplicationSettings.ICredentialCommand";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("a5f665e6-6143-4a7a-a971-b017ba978ce2")
                ICredentialCommand : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PasswordCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CredentialDeleted(
                        ABI::Windows::UI::ApplicationSettings::ICredentialCommandCredentialDeletedHandler** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICredentialCommand = __uuidof(ICredentialCommand);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ICredentialCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.CredentialCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ICredentialCommandFactory[] = L"Windows.UI.ApplicationSettings.ICredentialCommandFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("27e88c17-bc3e-4b80-9495-4ed720e48a91")
                ICredentialCommandFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCredentialCommand(
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        ABI::Windows::UI::ApplicationSettings::ICredentialCommand** instance
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCredentialCommandWithHandler(
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        ABI::Windows::UI::ApplicationSettings::ICredentialCommandCredentialDeletedHandler* deleted,
                        ABI::Windows::UI::ApplicationSettings::ICredentialCommand** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICredentialCommandFactory = __uuidof(ICredentialCommandFactory);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsCommandFactory[] = L"Windows.UI.ApplicationSettings.ISettingsCommandFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("68e15b33-1c83-433a-aa5a-ceeea5bd4764")
                ISettingsCommandFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateSettingsCommand(
                        IInspectable* settingsCommandId,
                        HSTRING label,
                        ABI::Windows::UI::Popups::IUICommandInvokedHandler* handler,
                        ABI::Windows::UI::Popups::IUICommand** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsCommandFactory = __uuidof(ISettingsCommandFactory);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsCommandStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsCommandStatics[] = L"Windows.UI.ApplicationSettings.ISettingsCommandStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("749ae954-2f69-4b17-8aba-d05ce5778e46")
                ISettingsCommandStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AccountsCommand(
                        ABI::Windows::UI::Popups::IUICommand** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsCommandStatics = __uuidof(ISettingsCommandStatics);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPane
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPane
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPane[] = L"Windows.UI.ApplicationSettings.ISettingsPane";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("b1cd0932-4570-4c69-8d38-89446561ace0")
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                ISettingsPane : public IInspectable
                {
                public:
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_CommandsRequested(
                        __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* handler,
                        EventRegistrationToken* cookie
                        ) = 0;
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_CommandsRequested(
                        EventRegistrationToken cookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsPane = __uuidof(ISettingsPane);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneCommandsRequest[] = L"Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("44df23ae-5d6e-4068-a168-f47643182114")
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                ISettingsPaneCommandsRequest : public IInspectable
                {
                public:
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationCommands(
                        __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsPaneCommandsRequest = __uuidof(ISettingsPaneCommandsRequest);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("205f5d24-1b48-4629-a6ca-2fdfedafb75d")
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                ISettingsPaneCommandsRequestedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Request(
                        ABI::Windows::UI::ApplicationSettings::ISettingsPaneCommandsRequest** request
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsPaneCommandsRequestedEventArgs = __uuidof(ISettingsPaneCommandsRequestedEventArgs);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneStatics
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPane
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneStatics[] = L"Windows.UI.ApplicationSettings.ISettingsPaneStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("1c6a52c5-ff19-471b-ba6b-f8f35694ad9a")
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                ISettingsPaneStatics : public IInspectable
                {
                public:
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::UI::ApplicationSettings::ISettingsPane** current
                        ) = 0;
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Show(void) = 0;
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Edge(
                        ABI::Windows::UI::ApplicationSettings::SettingsEdgeLocation* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISettingsPaneStatics = __uuidof(ISettingsPaneStatics);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountCommand[] = L"Windows.UI.ApplicationSettings.IWebAccountCommand";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("caa39398-9cfa-4246-b0c4-a913a3896541")
                IWebAccountCommand : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebAccount(
                        ABI::Windows::Security::Credentials::IWebAccount** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Invoked(
                        ABI::Windows::UI::ApplicationSettings::IWebAccountCommandInvokedHandler** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Actions(
                        ABI::Windows::UI::ApplicationSettings::SupportedWebAccountActions* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountCommand = __uuidof(IWebAccountCommand);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountCommandFactory[] = L"Windows.UI.ApplicationSettings.IWebAccountCommandFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("bfa6cdff-2f2d-42f5-81de-1d56bafc496d")
                IWebAccountCommandFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWebAccountCommand(
                        ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                        ABI::Windows::UI::ApplicationSettings::IWebAccountCommandInvokedHandler* invoked,
                        ABI::Windows::UI::ApplicationSettings::SupportedWebAccountActions actions,
                        ABI::Windows::UI::ApplicationSettings::IWebAccountCommand** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountCommandFactory = __uuidof(IWebAccountCommandFactory);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountInvokedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountInvokedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountInvokedArgs[] = L"Windows.UI.ApplicationSettings.IWebAccountInvokedArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("e7abcc40-a1d8-4c5d-9a7f-1d34b2f90ad2")
                IWebAccountInvokedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        ABI::Windows::UI::ApplicationSettings::WebAccountAction* action
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountInvokedArgs = __uuidof(IWebAccountInvokedArgs);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountProviderCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountProviderCommand[] = L"Windows.UI.ApplicationSettings.IWebAccountProviderCommand";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("d69bdd9a-a0a6-4e9b-88dc-c71e757a3501")
                IWebAccountProviderCommand : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebAccountProvider(
                        ABI::Windows::Security::Credentials::IWebAccountProvider** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Invoked(
                        ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommandInvokedHandler** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProviderCommand = __uuidof(IWebAccountProviderCommand);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountProviderCommandFactory[] = L"Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ApplicationSettings {
                MIDL_INTERFACE("d5658a1b-b176-4776-8469-a9d3ff0b3f59")
                IWebAccountProviderCommandFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWebAccountProviderCommand(
                        ABI::Windows::Security::Credentials::IWebAccountProvider* webAccountProvider,
                        ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommandInvokedHandler* invoked,
                        ABI::Windows::UI::ApplicationSettings::IWebAccountProviderCommand** instance
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWebAccountProviderCommandFactory = __uuidof(IWebAccountProviderCommandFactory);
            } /* ApplicationSettings */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPane ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPane_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPane[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPane";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs ** Default Interface **
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.CredentialCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.ICredentialCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ICredentialCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_CredentialCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_CredentialCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_CredentialCommand[] = L"Windows.UI.ApplicationSettings.CredentialCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.ISettingsCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.ISettingsCommandStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsCommand[] = L"Windows.UI.ApplicationSettings.SettingsCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPane
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.ISettingsPaneStatics interface starting with version 1.0 of the Windows.UI.ApplicationSettings.ApplicationsSettingsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPane ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPane_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPane[] = L"Windows.UI.ApplicationSettings.SettingsPane";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest[] = L"Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.IWebAccountCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountCommand[] = L"Windows.UI.ApplicationSettings.WebAccountCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountInvokedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountInvokedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountInvokedArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountInvokedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountInvokedArgs[] = L"Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountProviderCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountProviderCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountProviderCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountProviderCommand[] = L"Windows.UI.ApplicationSettings.WebAccountProviderCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2 __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2 __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3 __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory;

#endif // ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

typedef struct __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl;

interface __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

typedef struct __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __FIIterator_1_Windows__CUI__CApplicationSettings__CCredentialCommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl;

interface __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommand __x_ABI_CWindows_CUI_CPopups_CIUICommand;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommand_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

typedef struct __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl;

interface __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

typedef struct __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __FIIterator_1_Windows__CUI__CApplicationSettings__CSettingsCommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl;

interface __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

typedef struct __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl;

interface __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

typedef struct __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountCommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl;

interface __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

typedef struct __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl;

interface __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

typedef struct __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __FIIterator_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl;

interface __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

typedef struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl;

interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

typedef struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl;

interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

typedef struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl;

interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

typedef struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl;

interface __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand;

typedef struct __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __FIVectorView_1_Windows__CUI__CApplicationSettings__CCredentialCommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl;

interface __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand;

typedef struct __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __FIVectorView_1_Windows__CUI__CApplicationSettings__CSettingsCommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl;

interface __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand;

typedef struct __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountCommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl;

interface __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand;

typedef struct __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __FIVectorView_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl;

interface __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* sender,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* sender,
        __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler;

#endif // ____x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CApplicationSettings_CSettingsEdgeLocation __x_ABI_CWindows_CUI_CApplicationSettings_CSettingsEdgeLocation;

typedef enum __x_ABI_CWindows_CUI_CApplicationSettings_CSupportedWebAccountActions __x_ABI_CWindows_CUI_CApplicationSettings_CSupportedWebAccountActions;

typedef enum __x_ABI_CWindows_CUI_CApplicationSettings_CWebAccountAction __x_ABI_CWindows_CUI_CApplicationSettings_CWebAccountAction;

/*
 *
 * Struct Windows.UI.ApplicationSettings.SettingsEdgeLocation
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsEdgeLocation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CApplicationSettings_CSettingsEdgeLocation
{
    SettingsEdgeLocation_Right = 0,
    SettingsEdgeLocation_Left = 1,
};
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ApplicationSettings.SupportedWebAccountActions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CApplicationSettings_CSupportedWebAccountActions
{
    SupportedWebAccountActions_None = 0,
    SupportedWebAccountActions_Reconnect = 0x1,
    SupportedWebAccountActions_Remove = 0x2,
    SupportedWebAccountActions_ViewDetails = 0x4,
    SupportedWebAccountActions_Manage = 0x8,
    SupportedWebAccountActions_More = 0x10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.ApplicationSettings.WebAccountAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CApplicationSettings_CWebAccountAction
{
    WebAccountAction_Reconnect = 0,
    WebAccountAction_Remove = 1,
    WebAccountAction_ViewDetails = 2,
    WebAccountAction_Manage = 3,
    WebAccountAction_More = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.CredentialCommandCredentialDeletedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* command);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_Invoke(This, command) \
    ((This)->lpVtbl->Invoke(This, command))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.WebAccountCommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* command,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* args);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_Invoke(This, command, args) \
    ((This)->lpVtbl->Invoke(This, command, args))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.ApplicationSettings.WebAccountProviderCommandInvokedHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* command);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandlerVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_Invoke(This, command) \
    ((This)->lpVtbl->Invoke(This, command))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPane[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPane";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AccountCommandsRequested)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CAccountsSettingsPane_Windows__CUI__CApplicationSettings__CAccountsSettingsPaneCommandsRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_AccountCommandsRequested)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_add_AccountCommandsRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_AccountCommandsRequested(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_remove_AccountCommandsRequested(This, cookie) \
    ((This)->lpVtbl->remove_AccountCommandsRequested(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountProviderCommands)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountProviderCommand** value);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountCommands)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __FIVector_1_Windows__CUI__CApplicationSettings__CWebAccountCommand** value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialCommands)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __FIVector_1_Windows__CUI__CApplicationSettings__CCredentialCommand** value);
    HRESULT (STDMETHODCALLTYPE* get_Commands)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand** value);
    HRESULT (STDMETHODCALLTYPE* get_HeaderText)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_HeaderText)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_get_WebAccountProviderCommands(This, value) \
    ((This)->lpVtbl->get_WebAccountProviderCommands(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_get_WebAccountCommands(This, value) \
    ((This)->lpVtbl->get_WebAccountCommands(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_get_CredentialCommands(This, value) \
    ((This)->lpVtbl->get_CredentialCommands(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_get_Commands(This, value) \
    ((This)->lpVtbl->get_Commands(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_get_HeaderText(This, value) \
    ((This)->lpVtbl->get_HeaderText(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_put_HeaderText(This, value) \
    ((This)->lpVtbl->put_HeaderText(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneCommandsRequestedEventArgs2[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneCommandsRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneEventDeferral[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferralVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneEventDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPane** current);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStaticsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_GetForCurrentView(This, current) \
    ((This)->lpVtbl->GetForCurrentView(This, current))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_Show(This) \
    ((This)->lpVtbl->Show(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics2[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowManageAccountsAsync)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* ShowAddAccountAsync)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_ShowManageAccountsAsync(This, asyncInfo) \
    ((This)->lpVtbl->ShowManageAccountsAsync(This, asyncInfo))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_ShowAddAccountAsync(This, asyncInfo) \
    ((This)->lpVtbl->ShowAddAccountAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IAccountsSettingsPaneStatics3[] = L"Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowManageAccountsForUserAsync)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ShowAddAccountForUserAsync)(__x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_ShowManageAccountsForUserAsync(This, user, operation) \
    ((This)->lpVtbl->ShowManageAccountsForUserAsync(This, user, operation))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_ShowAddAccountForUserAsync(This, user, operation) \
    ((This)->lpVtbl->ShowAddAccountForUserAsync(This, user, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIAccountsSettingsPaneStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ICredentialCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.CredentialCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ICredentialCommand[] = L"Windows.UI.ApplicationSettings.ICredentialCommand";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PasswordCredential)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* get_CredentialDeleted)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_get_PasswordCredential(This, value) \
    ((This)->lpVtbl->get_PasswordCredential(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_get_CredentialDeleted(This, value) \
    ((This)->lpVtbl->get_CredentialDeleted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ICredentialCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.CredentialCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ICredentialCommandFactory[] = L"Windows.UI.ApplicationSettings.ICredentialCommandFactory";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCredentialCommand)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** instance);
    HRESULT (STDMETHODCALLTYPE* CreateCredentialCommandWithHandler)(__x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandCredentialDeletedHandler* deleted,
        __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommand** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactoryVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_CreateCredentialCommand(This, passwordCredential, instance) \
    ((This)->lpVtbl->CreateCredentialCommand(This, passwordCredential, instance))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_CreateCredentialCommandWithHandler(This, passwordCredential, deleted, instance) \
    ((This)->lpVtbl->CreateCredentialCommandWithHandler(This, passwordCredential, deleted, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CICredentialCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsCommandFactory[] = L"Windows.UI.ApplicationSettings.ISettingsCommandFactory";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateSettingsCommand)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory* This,
        IInspectable* settingsCommandId,
        HSTRING label,
        __x_ABI_CWindows_CUI_CPopups_CIUICommandInvokedHandler* handler,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactoryVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_CreateSettingsCommand(This, settingsCommandId, label, handler, instance) \
    ((This)->lpVtbl->CreateSettingsCommand(This, settingsCommandId, label, handler, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsCommandStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsCommandStatics[] = L"Windows.UI.ApplicationSettings.ISettingsCommandStatics";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AccountsCommand)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics* This,
        __x_ABI_CWindows_CUI_CPopups_CIUICommand** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStaticsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_get_AccountsCommand(This, value) \
    ((This)->lpVtbl->get_AccountsCommand(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsCommandStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPane
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPane
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPane[] = L"Windows.UI.ApplicationSettings.ISettingsPane";
typedef struct
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        TrustLevel* trustLevel);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_CommandsRequested)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        __FITypedEventHandler_2_Windows__CUI__CApplicationSettings__CSettingsPane_Windows__CUI__CApplicationSettings__CSettingsPaneCommandsRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_CommandsRequested)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_add_CommandsRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_CommandsRequested(This, handler, cookie))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_remove_CommandsRequested(This, cookie) \
    ((This)->lpVtbl->remove_CommandsRequested(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneCommandsRequest[] = L"Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest";
typedef struct
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This,
        TrustLevel* trustLevel);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ApplicationCommands)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest* This,
        __FIVector_1_Windows__CUI__CApplicationSettings__CSettingsCommand** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_get_ApplicationCommands(This, value) \
    ((This)->lpVtbl->get_ApplicationCommands(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs";
typedef struct
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequest** request);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_get_Request(This, request) \
    ((This)->lpVtbl->get_Request(This, request))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneCommandsRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.ISettingsPaneStatics
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.SettingsPane
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_ISettingsPaneStatics[] = L"Windows.UI.ApplicationSettings.ISettingsPaneStatics";
typedef struct
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPane** current);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This);
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Edge)(__x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics* This,
        enum __x_ABI_CWindows_CUI_CApplicationSettings_CSettingsEdgeLocation* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStaticsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_GetForCurrentView(This, current) \
    ((This)->lpVtbl->GetForCurrentView(This, current))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_Show(This) \
    ((This)->lpVtbl->Show(This))

#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
    DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_get_Edge(This, value) \
    ((This)->lpVtbl->get_Edge(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CISettingsPaneStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountCommand[] = L"Windows.UI.ApplicationSettings.IWebAccountCommand";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccount)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount** value);
    HRESULT (STDMETHODCALLTYPE* get_Invoked)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler** value);
    HRESULT (STDMETHODCALLTYPE* get_Actions)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand* This,
        enum __x_ABI_CWindows_CUI_CApplicationSettings_CSupportedWebAccountActions* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_get_WebAccount(This, value) \
    ((This)->lpVtbl->get_WebAccount(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_get_Invoked(This, value) \
    ((This)->lpVtbl->get_Invoked(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_get_Actions(This, value) \
    ((This)->lpVtbl->get_Actions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountCommandFactory[] = L"Windows.UI.ApplicationSettings.IWebAccountCommandFactory";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWebAccountCommand)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandInvokedHandler* invoked,
        enum __x_ABI_CWindows_CUI_CApplicationSettings_CSupportedWebAccountActions actions,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommand** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactoryVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_CreateWebAccountCommand(This, webAccount, invoked, actions, instance) \
    ((This)->lpVtbl->CreateWebAccountCommand(This, webAccount, invoked, actions, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountInvokedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountInvokedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountInvokedArgs[] = L"Windows.UI.ApplicationSettings.IWebAccountInvokedArgs";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs* This,
        enum __x_ABI_CWindows_CUI_CApplicationSettings_CWebAccountAction* action);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgsVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_get_Action(This, action) \
    ((This)->lpVtbl->get_Action(This, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountInvokedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountProviderCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountProviderCommand[] = L"Windows.UI.ApplicationSettings.IWebAccountProviderCommand";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebAccountProvider)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider** value);
    HRESULT (STDMETHODCALLTYPE* get_Invoked)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand* This,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_get_WebAccountProvider(This, value) \
    ((This)->lpVtbl->get_WebAccountProvider(This, value))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_get_Invoked(This, value) \
    ((This)->lpVtbl->get_Invoked(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_ApplicationSettings_IWebAccountProviderCommandFactory[] = L"Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory";
typedef struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWebAccountProviderCommand)(__x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccountProvider* webAccountProvider,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandInvokedHandler* invoked,
        __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommand** instance);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactoryVtbl;

interface __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_CreateWebAccountProviderCommand(This, webAccountProvider, invoked, instance) \
    ((This)->lpVtbl->CreateWebAccountProviderCommand(This, webAccountProvider, invoked, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CApplicationSettings_CIWebAccountProviderCommandFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPane
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPane ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPane_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPane[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPane";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs ** Default Interface **
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPaneCommandsRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_AccountsSettingsPaneEventDeferral[] = L"Windows.UI.ApplicationSettings.AccountsSettingsPaneEventDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.CredentialCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.ICredentialCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ICredentialCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_CredentialCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_CredentialCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_CredentialCommand[] = L"Windows.UI.ApplicationSettings.CredentialCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.ISettingsCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.ISettingsCommandStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Popups.IUICommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsCommand[] = L"Windows.UI.ApplicationSettings.SettingsCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPane
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.ApplicationSettings.ISettingsPaneStatics interface starting with version 1.0 of the Windows.UI.ApplicationSettings.ApplicationsSettingsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPane ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPane_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPane_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPane is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPane[] = L"Windows.UI.ApplicationSettings.SettingsPane";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequest is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequest[] = L"Windows.UI.ApplicationSettings.SettingsPaneCommandsRequest";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs
 *
 * Introduced to Windows.UI.ApplicationSettings.ApplicationsSettingsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs_DEFINED
#if WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
DEPRECATED("SettingsPaneCommandsRequestedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_SettingsPaneCommandsRequestedEventArgs[] = L"Windows.UI.ApplicationSettings.SettingsPaneCommandsRequestedEventArgs";
#endif
#endif // WINDOWS_UI_APPLICATIONSETTINGS_APPLICATIONSSETTINGSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.IWebAccountCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountCommand[] = L"Windows.UI.ApplicationSettings.WebAccountCommand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountInvokedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountInvokedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountInvokedArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountInvokedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountInvokedArgs[] = L"Windows.UI.ApplicationSettings.WebAccountInvokedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.ApplicationSettings.WebAccountProviderCommand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.ApplicationSettings.IWebAccountProviderCommand ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountProviderCommand_DEFINED
#define RUNTIMECLASS_Windows_UI_ApplicationSettings_WebAccountProviderCommand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ApplicationSettings_WebAccountProviderCommand[] = L"Windows.UI.ApplicationSettings.WebAccountProviderCommand";
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
#endif // __windows2Eui2Eapplicationsettings_p_h__

#endif // __windows2Eui2Eapplicationsettings_h__
