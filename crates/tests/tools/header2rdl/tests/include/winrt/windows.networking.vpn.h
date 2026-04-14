
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
#ifndef __windows2Enetworking2Evpn_h__
#define __windows2Enetworking2Evpn_h__
#ifndef __windows2Enetworking2Evpn_p_h__
#define __windows2Enetworking2Evpn_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)
#define WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_NETWORKING_SOCKETS_CONTROLCHANNELTRIGGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.Networking.h"
#include "Windows.Networking.Sockets.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Security.Cryptography.Certificates.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnAppId;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId ABI::Windows::Networking::Vpn::IVpnAppId

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnAppIdFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory ABI::Windows::Networking::Vpn::IVpnAppIdFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannel;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel ABI::Windows::Networking::Vpn::IVpnChannel

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannel2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2 ABI::Windows::Networking::Vpn::IVpnChannel2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannel4;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4 ABI::Windows::Networking::Vpn::IVpnChannel4

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannel5;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5 ABI::Windows::Networking::Vpn::IVpnChannel5

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannel6;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6 ABI::Windows::Networking::Vpn::IVpnChannel6

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannelActivityEventArgs;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs ABI::Windows::Networking::Vpn::IVpnChannelActivityEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannelActivityStateChangedArgs;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs ABI::Windows::Networking::Vpn::IVpnChannelActivityStateChangedArgs

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannelConfiguration;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration ABI::Windows::Networking::Vpn::IVpnChannelConfiguration

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannelConfiguration2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2 ABI::Windows::Networking::Vpn::IVpnChannelConfiguration2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnChannelStatics;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics ABI::Windows::Networking::Vpn::IVpnChannelStatics

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCredential;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential ABI::Windows::Networking::Vpn::IVpnCredential

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomCheckBox;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox ABI::Windows::Networking::Vpn::IVpnCustomCheckBox

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomComboBox;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox ABI::Windows::Networking::Vpn::IVpnCustomComboBox

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomEditBox;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox ABI::Windows::Networking::Vpn::IVpnCustomEditBox

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomErrorBox;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox ABI::Windows::Networking::Vpn::IVpnCustomErrorBox

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPrompt;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt ABI::Windows::Networking::Vpn::IVpnCustomPrompt

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPromptBooleanInput;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput ABI::Windows::Networking::Vpn::IVpnCustomPromptBooleanInput

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPromptElement;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement ABI::Windows::Networking::Vpn::IVpnCustomPromptElement

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPromptOptionSelector;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector ABI::Windows::Networking::Vpn::IVpnCustomPromptOptionSelector

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPromptText;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText ABI::Windows::Networking::Vpn::IVpnCustomPromptText

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomPromptTextInput;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput ABI::Windows::Networking::Vpn::IVpnCustomPromptTextInput

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnCustomTextBox;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox ABI::Windows::Networking::Vpn::IVpnCustomTextBox

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnDomainNameAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment ABI::Windows::Networking::Vpn::IVpnDomainNameAssignment

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnDomainNameInfo;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo ABI::Windows::Networking::Vpn::IVpnDomainNameInfo

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnDomainNameInfo2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2 ABI::Windows::Networking::Vpn::IVpnDomainNameInfo2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnDomainNameInfoFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory ABI::Windows::Networking::Vpn::IVpnDomainNameInfoFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnForegroundActivatedEventArgs;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs ABI::Windows::Networking::Vpn::IVpnForegroundActivatedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnForegroundActivationOperation;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation ABI::Windows::Networking::Vpn::IVpnForegroundActivationOperation

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnInterfaceId;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId ABI::Windows::Networking::Vpn::IVpnInterfaceId

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnInterfaceIdFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory ABI::Windows::Networking::Vpn::IVpnInterfaceIdFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnManagementAgent;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent ABI::Windows::Networking::Vpn::IVpnManagementAgent

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnNamespaceAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment ABI::Windows::Networking::Vpn::IVpnNamespaceAssignment

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnNamespaceInfo;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo ABI::Windows::Networking::Vpn::IVpnNamespaceInfo

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnNamespaceInfoFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory ABI::Windows::Networking::Vpn::IVpnNamespaceInfoFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnNativeProfile;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile ABI::Windows::Networking::Vpn::IVpnNativeProfile

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnNativeProfile2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2 ABI::Windows::Networking::Vpn::IVpnNativeProfile2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBuffer;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer ABI::Windows::Networking::Vpn::IVpnPacketBuffer

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBuffer2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2 ABI::Windows::Networking::Vpn::IVpnPacketBuffer2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBuffer3;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3 ABI::Windows::Networking::Vpn::IVpnPacketBuffer3

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBufferFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory ABI::Windows::Networking::Vpn::IVpnPacketBufferFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBufferList;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList ABI::Windows::Networking::Vpn::IVpnPacketBufferList

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPacketBufferList2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2 ABI::Windows::Networking::Vpn::IVpnPacketBufferList2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPickedCredential;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential ABI::Windows::Networking::Vpn::IVpnPickedCredential

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPlugIn;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn ABI::Windows::Networking::Vpn::IVpnPlugIn

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPlugInProfile;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile ABI::Windows::Networking::Vpn::IVpnPlugInProfile

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPlugInProfile2;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2 ABI::Windows::Networking::Vpn::IVpnPlugInProfile2

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnPlugInReconnectTransport;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport ABI::Windows::Networking::Vpn::IVpnPlugInReconnectTransport

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnProfile;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile ABI::Windows::Networking::Vpn::IVpnProfile

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnRoute;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute ABI::Windows::Networking::Vpn::IVpnRoute

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnRouteAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment ABI::Windows::Networking::Vpn::IVpnRouteAssignment

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnRouteFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory ABI::Windows::Networking::Vpn::IVpnRouteFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnSystemHealth;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth ABI::Windows::Networking::Vpn::IVpnSystemHealth

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnTrafficFilter;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter ABI::Windows::Networking::Vpn::IVpnTrafficFilter

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnTrafficFilterAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment ABI::Windows::Networking::Vpn::IVpnTrafficFilterAssignment

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                interface IVpnTrafficFilterFactory;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory ABI::Windows::Networking::Vpn::IVpnTrafficFilterFactory

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("91ccb492-ec28-530b-b45e-c431744ca9b5"))
IIterator<ABI::Windows::Networking::Vpn::IVpnProfile*> : IIterator_impl<ABI::Windows::Networking::Vpn::IVpnProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.IVpnProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::IVpnProfile*> __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db35f6b1-f266-5c87-8862-9dd87d9df18f"))
IIterable<ABI::Windows::Networking::Vpn::IVpnProfile*> : IIterable_impl<ABI::Windows::Networking::Vpn::IVpnProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.IVpnProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::IVpnProfile*> __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f1dc8f7d-ca33-53fd-9d4c-40c51b5873ec"))
IVectorView<ABI::Windows::Networking::Vpn::IVpnProfile*> : IVectorView_impl<ABI::Windows::Networking::Vpn::IVpnProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.IVpnProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::IVpnProfile*> __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("69d957be-045e-538f-98f6-1aa65cee244a"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.IVpnProfile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("dac6dd72-a5d1-56d4-afc4-989f84dcb2b3"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.IVpnProfile>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnCredential;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("216a6f97-dba1-5f71-a14b-2818ad3c4c69"))
IAsyncOperation<ABI::Windows::Networking::Vpn::VpnCredential*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnCredential*, ABI::Windows::Networking::Vpn::IVpnCredential*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Vpn.VpnCredential>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::Vpn::VpnCredential*> __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1c9c4504-4b75-57ea-837d-5338358bb762"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::Vpn::VpnCredential*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnCredential*, ABI::Windows::Networking::Vpn::IVpnCredential*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Vpn.VpnCredential>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::Vpn::VpnCredential*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnManagementErrorStatus : int VpnManagementErrorStatus;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e99b86dc-6b65-5f23-9419-90b55852f283"))
IAsyncOperation<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus> : IAsyncOperation_impl<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.Vpn.VpnManagementErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus> __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("31229f8c-709d-5017-8629-57ef1289e616"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.Vpn.VpnManagementErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Networking::Vpn::VpnManagementErrorStatus> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_IInspectable_USE
#define DEF___FIIterator_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44a94f2d-04f8-5091-b336-be7892dd10be"))
IIterator<IInspectable*> : IIterator_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<IInspectable*> __FIIterator_1_IInspectable_t;
#define __FIIterator_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_IInspectable_USE */



#ifndef DEF___FIIterable_1_IInspectable_USE
#define DEF___FIIterable_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("092b849b-60b1-52be-a44a-6fe8e933cbe4"))
IIterable<IInspectable*> : IIterable_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<IInspectable*> __FIIterable_1_IInspectable_t;
#define __FIIterable_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_IInspectable_USE */



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

#ifndef DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c157d0f-5efe-5cec-bbd6-0c6ce9af07a5"))
IIterator<ABI::Windows::Foundation::Uri*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Uri*> __FIIterator_1_Windows__CFoundation__CUri_t;
#define __FIIterator_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0d63b78-78ad-5e31-b6d8-e32a0e16c447"))
IIterable<ABI::Windows::Foundation::Uri*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Uri*> __FIIterable_1_Windows__CFoundation__CUri_t;
#define __FIIterable_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            class HostName;
        } /* Networking */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            interface IHostName;
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CIHostName ABI::Windows::Networking::IHostName

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CHostName_USE
#define DEF___FIIterator_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("557bf83c-a428-5dbd-a0fe-05f6ee543d45"))
IIterator<ABI::Windows::Networking::HostName*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::HostName*> __FIIterator_1_Windows__CNetworking__CHostName_t;
#define __FIIterator_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CHostName_USE
#define DEF___FIIterable_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9e5f3ed0-cf1c-5d38-832c-acea6164bf5c"))
IIterable<ABI::Windows::Networking::HostName*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::HostName*> __FIIterable_1_Windows__CNetworking__CHostName_t;
#define __FIIterable_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6fc7cfe8-1882-5ba1-8e5e-4c5e3a4fa86d"))
IIterator<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> : IIterator_impl<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.IVpnCustomPrompt>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8bac695c-70cb-54d6-8606-af6e3a25e3a1"))
IIterable<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> : IIterable_impl<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.IVpnCustomPrompt>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee23ff21-51ba-5cc4-9856-625c79c28080"))
IIterator<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> : IIterator_impl<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.IVpnCustomPromptElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("437d3693-00c4-50b4-989a-938f1016a230"))
IIterable<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> : IIterable_impl<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.IVpnCustomPromptElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnAppId;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ec9298b0-8ca2-549c-bbe2-252823e70eef"))
IIterator<ABI::Windows::Networking::Vpn::VpnAppId*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnAppId*, ABI::Windows::Networking::Vpn::IVpnAppId*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnAppId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnAppId*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0e1e00aa-f93d-5dc7-9912-e07d1fa6bd67"))
IIterable<ABI::Windows::Networking::Vpn::VpnAppId*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnAppId*, ABI::Windows::Networking::Vpn::IVpnAppId*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnAppId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnAppId*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnDomainNameInfo;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("29f9008e-3e81-5c58-8a78-6be91abcc17d"))
IIterator<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*, ABI::Windows::Networking::Vpn::IVpnDomainNameInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnDomainNameInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("33abe488-be1a-558a-a9cf-b5330ab49f50"))
IIterable<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*, ABI::Windows::Networking::Vpn::IVpnDomainNameInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnDomainNameInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnNamespaceInfo;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("91e28244-7555-594d-b54d-9c87095e79a2"))
IIterator<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*, ABI::Windows::Networking::Vpn::IVpnNamespaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnNamespaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("17781d03-ddcf-553f-aba8-d2e8155cb6b8"))
IIterable<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*, ABI::Windows::Networking::Vpn::IVpnNamespaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnNamespaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnPacketBuffer;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("04c88ba5-05e9-53b2-8524-df458d2a9179"))
IIterator<ABI::Windows::Networking::Vpn::VpnPacketBuffer*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnPacketBuffer*, ABI::Windows::Networking::Vpn::IVpnPacketBuffer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnPacketBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnPacketBuffer*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fa954f6e-30d7-50e0-9d43-dadb6c53e196"))
IIterable<ABI::Windows::Networking::Vpn::VpnPacketBuffer*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnPacketBuffer*, ABI::Windows::Networking::Vpn::IVpnPacketBuffer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnPacketBuffer*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnRoute;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44f813ac-052f-514b-a776-aad37a64fdc6"))
IIterator<ABI::Windows::Networking::Vpn::VpnRoute*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnRoute*, ABI::Windows::Networking::Vpn::IVpnRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnRoute*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bbf498d5-b9ef-55f1-97b1-77a06639e4e2"))
IIterable<ABI::Windows::Networking::Vpn::VpnRoute*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnRoute*, ABI::Windows::Networking::Vpn::IVpnRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnRoute*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnTrafficFilter;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#define DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8f37d5a0-5905-55bf-9c18-b9b3b544648b"))
IIterator<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnTrafficFilter*, ABI::Windows::Networking::Vpn::IVpnTrafficFilter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.Vpn.VpnTrafficFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t;
#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#define DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6d4c853-77c9-52ca-9ce9-853add4554cf"))
IIterable<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnTrafficFilter*, ABI::Windows::Networking::Vpn::IVpnTrafficFilter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnTrafficFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t;
#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b8385bd-a2cd-5ff1-bf74-7ea580423e50"))
IVectorView<ABI::Windows::Foundation::Uri*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::Uri*> __FIVectorView_1_Windows__CFoundation__CUri_t;
#define __FIVectorView_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f4706ab1-55a3-5270-afb2-732988fe8227"))
IVectorView<ABI::Windows::Networking::HostName*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::HostName*> __FIVectorView_1_Windows__CNetworking__CHostName_t;
#define __FIVectorView_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("804449c2-3bc1-5cfe-8468-3bbece4a5cd7"))
IVectorView<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> : IVectorView_impl<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.IVpnCustomPrompt>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::IVpnCustomPrompt*> __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("25eaf5eb-2f39-5b4d-bb6e-e652c7d00f6a"))
IVectorView<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> : IVectorView_impl<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.IVpnCustomPromptElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::IVpnCustomPromptElement*> __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("faecbc50-da9a-5102-8229-1dd24e873d1f"))
IVectorView<ABI::Windows::Networking::Vpn::VpnAppId*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnAppId*, ABI::Windows::Networking::Vpn::IVpnAppId*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.VpnAppId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::VpnAppId*> __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d4772f57-2328-5c25-9a11-246da17e39d5"))
IVectorView<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*, ABI::Windows::Networking::Vpn::IVpnDomainNameInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.VpnDomainNameInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c6ed05a9-4dc7-507d-9c92-7c78c2ef4786"))
IVectorView<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*, ABI::Windows::Networking::Vpn::IVpnNamespaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.VpnNamespaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("43701a74-e497-5559-a71b-11d0156fa839"))
IVectorView<ABI::Windows::Networking::Vpn::VpnRoute*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnRoute*, ABI::Windows::Networking::Vpn::IVpnRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.VpnRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::VpnRoute*> __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("75de1766-ee22-56e9-be99-2714065349e5"))
IVectorView<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnTrafficFilter*, ABI::Windows::Networking::Vpn::IVpnTrafficFilter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.Vpn.VpnTrafficFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t;
#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CFoundation__CUri_USE
#define DEF___FIVector_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d82bd8d-fe62-5d67-a7b9-7886dd75bc4e"))
IVector<ABI::Windows::Foundation::Uri*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Foundation::Uri*> __FIVector_1_Windows__CFoundation__CUri_t;
#define __FIVector_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CHostName_USE
#define DEF___FIVector_1_Windows__CNetworking__CHostName_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("90c71c29-a9b5-5267-a5ad-8b756736317c"))
IVector<ABI::Windows::Networking::HostName*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::HostName*, ABI::Windows::Networking::IHostName*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.HostName>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::HostName*> __FIVector_1_Windows__CNetworking__CHostName_t;
#define __FIVector_1_Windows__CNetworking__CHostName ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CHostName_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CHostName_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#define DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("89097d58-edb8-5ad4-abc5-603f21dd4b15"))
IVector<ABI::Windows::Networking::Vpn::VpnAppId*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnAppId*, ABI::Windows::Networking::Vpn::IVpnAppId*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.Vpn.VpnAppId>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::Vpn::VpnAppId*> __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_t;
#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#define DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8179b6f2-7273-5ca3-a81b-53e902ca209b"))
IVector<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*, ABI::Windows::Networking::Vpn::IVpnDomainNameInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.Vpn.VpnDomainNameInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::Vpn::VpnDomainNameInfo*> __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t;
#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#define DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("056bddf2-135d-542e-a322-36aa4ca0e60d"))
IVector<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*, ABI::Windows::Networking::Vpn::IVpnNamespaceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.Vpn.VpnNamespaceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::Vpn::VpnNamespaceInfo*> __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t;
#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#define DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5b026bd8-2cc5-5570-bde5-0db7c4331279"))
IVector<ABI::Windows::Networking::Vpn::VpnRoute*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnRoute*, ABI::Windows::Networking::Vpn::IVpnRoute*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.Vpn.VpnRoute>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::Vpn::VpnRoute*> __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_t;
#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#define DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2a5b9ad8-f005-5f69-ad81-300642e7c667"))
IVector<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnTrafficFilter*, ABI::Windows::Networking::Vpn::IVpnTrafficFilter*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.Vpn.VpnTrafficFilter>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Networking::Vpn::VpnTrafficFilter*> __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t;
#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnChannel;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnChannelActivityEventArgs;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("39907949-a8cc-5ce5-9e0a-06e3b2d31570"))
ITypedEventHandler<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::VpnChannelActivityEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::IVpnChannel*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnChannelActivityEventArgs*, ABI::Windows::Networking::Vpn::IVpnChannelActivityEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Vpn.VpnChannel, Windows.Networking.Vpn.VpnChannelActivityEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::VpnChannelActivityEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnChannelActivityStateChangedArgs;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2cfcf1ba-bffc-5746-b868-2e45a46d2958"))
ITypedEventHandler<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::VpnChannelActivityStateChangedArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::IVpnChannel*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::Vpn::VpnChannelActivityStateChangedArgs*, ABI::Windows::Networking::Vpn::IVpnChannelActivityStateChangedArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.Vpn.VpnChannel, Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::Vpn::VpnChannel*, ABI::Windows::Networking::Vpn::VpnChannelActivityStateChangedArgs*> __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgsWithUser;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser ABI::Windows::ApplicationModel::Activation::IActivatedEventArgsWithUser

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

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
        namespace Networking {
            namespace Sockets {
                typedef enum ControlChannelTriggerStatus : int ControlChannelTriggerStatus;
            } /* Sockets */
        } /* Networking */
    } /* Windows */
} /* ABI */

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
            namespace Cryptography {
                namespace Certificates {
                    class Certificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate ABI::Windows::Security::Cryptography::Certificates::ICertificate

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class Buffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

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
        namespace Networking {
            namespace Vpn {
                typedef enum VpnAppIdType : int VpnAppIdType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnAuthenticationMethod : int VpnAuthenticationMethod;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnChannelActivityEventType : int VpnChannelActivityEventType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnCredentialType : int VpnCredentialType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnDataPathType : int VpnDataPathType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnDomainNameType : int VpnDomainNameType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnIPProtocol : int VpnIPProtocol;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnManagementConnectionStatus : int VpnManagementConnectionStatus;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnNativeProtocolType : int VpnNativeProtocolType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnPacketBufferStatus : int VpnPacketBufferStatus;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                typedef enum VpnRoutingPolicyType : int VpnRoutingPolicyType;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnChannelConfiguration;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnDomainNameAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnForegroundActivationOperation;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnInterfaceId;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnNamespaceAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnPacketBufferList;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnPickedCredential;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnRouteAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnSystemHealth;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                class VpnTrafficFilterAssignment;
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.Vpn.VpnAppIdType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnAppIdType : int
                {
                    VpnAppIdType_PackageFamilyName = 0,
                    VpnAppIdType_FullyQualifiedBinaryName = 1,
                    VpnAppIdType_FilePath = 2,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnAuthenticationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnAuthenticationMethod : int
                {
                    VpnAuthenticationMethod_Mschapv2 = 0,
                    VpnAuthenticationMethod_Eap = 1,
                    VpnAuthenticationMethod_Certificate = 2,
                    VpnAuthenticationMethod_PresharedKey = 3,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnChannelActivityEventType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnChannelActivityEventType : int
                {
                    VpnChannelActivityEventType_Idle = 0,
                    VpnChannelActivityEventType_Active = 1,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnChannelRequestCredentialsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnChannelRequestCredentialsOptions : unsigned int
                {
                    VpnChannelRequestCredentialsOptions_None = 0,
                    VpnChannelRequestCredentialsOptions_Retrying = 0x1,
                    VpnChannelRequestCredentialsOptions_UseForSingleSignIn = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(VpnChannelRequestCredentialsOptions)
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnCredentialType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnCredentialType : int
                {
                    VpnCredentialType_UsernamePassword = 0,
                    VpnCredentialType_UsernameOtpPin = 1,
                    VpnCredentialType_UsernamePasswordAndPin = 2,
                    VpnCredentialType_UsernamePasswordChange = 3,
                    VpnCredentialType_SmartCard = 4,
                    VpnCredentialType_ProtectedCertificate = 5,
                    VpnCredentialType_UnProtectedCertificate = 6,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnDataPathType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnDataPathType : int
                {
                    VpnDataPathType_Send = 0,
                    VpnDataPathType_Receive = 1,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnDomainNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnDomainNameType : int
                {
                    VpnDomainNameType_Suffix = 0,
                    VpnDomainNameType_FullyQualified = 1,
                    VpnDomainNameType_Reserved = 65535,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnIPProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnIPProtocol : int
                {
                    VpnIPProtocol_None = 0,
                    VpnIPProtocol_Tcp = 6,
                    VpnIPProtocol_Udp = 17,
                    VpnIPProtocol_Icmp = 1,
                    VpnIPProtocol_Ipv6Icmp = 58,
                    VpnIPProtocol_Igmp = 2,
                    VpnIPProtocol_Pgm = 113,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnManagementConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnManagementConnectionStatus : int
                {
                    VpnManagementConnectionStatus_Disconnected = 0,
                    VpnManagementConnectionStatus_Disconnecting = 1,
                    VpnManagementConnectionStatus_Connected = 2,
                    VpnManagementConnectionStatus_Connecting = 3,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnManagementErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnManagementErrorStatus : int
                {
                    VpnManagementErrorStatus_Ok = 0,
                    VpnManagementErrorStatus_Other = 1,
                    VpnManagementErrorStatus_InvalidXmlSyntax = 2,
                    VpnManagementErrorStatus_ProfileNameTooLong = 3,
                    VpnManagementErrorStatus_ProfileInvalidAppId = 4,
                    VpnManagementErrorStatus_AccessDenied = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_CannotFindProfile = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_AlreadyDisconnecting = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_AlreadyConnected = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_GeneralAuthenticationFailure = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_EapFailure = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_SmartCardFailure = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_CertificateFailure = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_ServerConfiguration = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_NoConnection = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_ServerConnection = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_UserNamePassword = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_DnsNotResolvable = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    VpnManagementErrorStatus_InvalidIP = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnNativeProtocolType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnNativeProtocolType : int
                {
                    VpnNativeProtocolType_Pptp = 0,
                    VpnNativeProtocolType_L2tp = 1,
                    VpnNativeProtocolType_IpsecIkev2 = 2,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnPacketBufferStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnPacketBufferStatus : int
                {
                    VpnPacketBufferStatus_Ok = 0,
                    VpnPacketBufferStatus_InvalidBufferSize = 1,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnRoutingPolicyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                enum VpnRoutingPolicyType : int
                {
                    VpnRoutingPolicyType_SplitRouting = 0,
                    VpnRoutingPolicyType_ForceAllTrafficOverVpn = 1,
                };
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnAppId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnAppId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnAppId[] = L"Windows.Networking.Vpn.IVpnAppId";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("7b06a635-5c58-41d9-94a7-bfbcf1d8ca54")
                IVpnAppId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Networking::Vpn::VpnAppIdType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Type(
                        ABI::Windows::Networking::Vpn::VpnAppIdType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Value(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnAppId = __uuidof(IVpnAppId);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnAppIdFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnAppId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnAppIdFactory[] = L"Windows.Networking.Vpn.IVpnAppIdFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("46adfd2a-0aab-4fdb-821d-d3ddc919788b")
                IVpnAppIdFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Networking::Vpn::VpnAppIdType type,
                        HSTRING value,
                        ABI::Windows::Networking::Vpn::IVpnAppId** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnAppIdFactory = __uuidof(IVpnAppIdFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel[] = L"Windows.Networking.Vpn.IVpnChannel";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("4ac78d07-d1a8-4303-a091-c8d2e0915bc3")
                IVpnChannel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AssociateTransport(
                        IInspectable* mainOuterTunnelTransport,
                        IInspectable* optionalOuterTunnelTransport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId* vpnInterfaceId,
                        ABI::Windows::Networking::Vpn::IVpnRouteAssignment* routeScope,
                        ABI::Windows::Networking::Vpn::IVpnNamespaceAssignment* namespaceScope,
                        UINT32 mtuSize,
                        UINT32 maxFrameSize,
                        boolean optimizeForLowCostNetwork,
                        IInspectable* mainOuterTunnelTransport,
                        IInspectable* optionalOuterTunnelTransport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCredentials(
                        ABI::Windows::Networking::Vpn::VpnCredentialType credType,
                        boolean isRetry,
                        boolean isSingleSignOnCredential,
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                        ABI::Windows::Networking::Vpn::IVpnPickedCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestVpnPacketBuffer(
                        ABI::Windows::Networking::Vpn::VpnDataPathType type,
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** vpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LogDiagnosticMessage(
                        HSTRING message
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::Networking::Vpn::IVpnChannelConfiguration** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ActivityChange(
                        __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ActivityChange(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PlugInContext(
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PlugInContext(
                        IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SystemHealth(
                        ABI::Windows::Networking::Vpn::IVpnSystemHealth** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCustomPrompt(
                        __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* customPrompt
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetErrorMessage(
                        HSTRING message
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetAllowedSslTlsVersions(
                        IInspectable* tunnelTransport,
                        boolean useTls12
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannel = __uuidof(IVpnChannel);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel2[] = L"Windows.Networking.Vpn.IVpnChannel2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("2255d165-993b-4629-ad60-f1c3f3537f50")
                IVpnChannel2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE StartWithMainTransport(
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId* vpnInterfaceId,
                        ABI::Windows::Networking::Vpn::IVpnRouteAssignment* assignedRoutes,
                        ABI::Windows::Networking::Vpn::IVpnDomainNameAssignment* assignedDomainName,
                        UINT32 mtuSize,
                        UINT32 maxFrameSize,
                        boolean Reserved,
                        IInspectable* mainOuterTunnelTransport
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartExistingTransports(
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId* vpnInterfaceId,
                        ABI::Windows::Networking::Vpn::IVpnRouteAssignment* assignedRoutes,
                        ABI::Windows::Networking::Vpn::IVpnDomainNameAssignment* assignedDomainName,
                        UINT32 mtuSize,
                        UINT32 maxFrameSize,
                        boolean Reserved
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ActivityStateChange(
                        __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ActivityStateChange(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVpnSendPacketBuffer(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** vpnSendPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVpnReceivePacketBuffer(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** vpnReceivePacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCustomPromptAsync(
                        __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* customPromptElement,
                        ABI::Windows::Foundation::IAsyncAction** action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCredentialsWithCertificateAsync(
                        ABI::Windows::Networking::Vpn::VpnCredentialType credType,
                        UINT32 credOptions,
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate* certificate,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCredentialsWithOptionsAsync(
                        ABI::Windows::Networking::Vpn::VpnCredentialType credType,
                        UINT32 credOptions,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestCredentialsSimpleAsync(
                        ABI::Windows::Networking::Vpn::VpnCredentialType credType,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TerminateConnection(
                        HSTRING message
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartWithTrafficFilter(
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIpv4List,
                        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIpv6List,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId* vpnInterfaceId,
                        ABI::Windows::Networking::Vpn::IVpnRouteAssignment* assignedRoutes,
                        ABI::Windows::Networking::Vpn::IVpnDomainNameAssignment* assignedNamespace,
                        UINT32 mtuSize,
                        UINT32 maxFrameSize,
                        boolean reserved,
                        IInspectable* mainOuterTunnelTransport,
                        IInspectable* optionalOuterTunnelTransport,
                        ABI::Windows::Networking::Vpn::IVpnTrafficFilterAssignment* assignedTrafficFilters
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannel2 = __uuidof(IVpnChannel2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel4[] = L"Windows.Networking.Vpn.IVpnChannel4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("d7266ede-2937-419d-9570-486aebb81803")
                IVpnChannel4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddAndAssociateTransport(
                        IInspectable* transport,
                        IInspectable* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartWithMultipleTransports(
                        __FIIterable_1_Windows__CNetworking__CHostName* assignedClientIpv4Addresses,
                        __FIIterable_1_Windows__CNetworking__CHostName* assignedClientIpv6Addresses,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId* vpninterfaceId,
                        ABI::Windows::Networking::Vpn::IVpnRouteAssignment* assignedRoutes,
                        ABI::Windows::Networking::Vpn::IVpnDomainNameAssignment* assignedNamespace,
                        UINT32 mtuSize,
                        UINT32 maxFrameSize,
                        boolean reserved,
                        __FIIterable_1_IInspectable* transports,
                        ABI::Windows::Networking::Vpn::IVpnTrafficFilterAssignment* assignedTrafficFilters
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReplaceAndAssociateTransport(
                        IInspectable* transport,
                        IInspectable* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartReconnectingTransport(
                        IInspectable* transport,
                        IInspectable* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSlotTypeForTransportContext(
                        IInspectable* context,
                        ABI::Windows::Networking::Sockets::ControlChannelTriggerStatus* slotType
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentRequestTransportContext(
                        IInspectable** context
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannel4 = __uuidof(IVpnChannel4);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel5[] = L"Windows.Networking.Vpn.IVpnChannel5";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("de7a0992-8384-4fbc-882c-1fd23124cd3b")
                IVpnChannel5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AppendVpnReceivePacketBuffer(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* decapsulatedPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AppendVpnSendPacketBuffer(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* encapsulatedPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushVpnReceivePacketBuffers(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FlushVpnSendPacketBuffers(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannel5 = __uuidof(IVpnChannel5);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel6[] = L"Windows.Networking.Vpn.IVpnChannel6";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("55843696-bd63-49c5-abca-5da77885551a")
                IVpnChannel6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ActivateForeground(
                        HSTRING packageRelativeAppId,
                        ABI::Windows::Foundation::Collections::IPropertySet* sharedContext,
                        ABI::Windows::Foundation::Collections::IPropertySet** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannel6 = __uuidof(IVpnChannel6);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelActivityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelActivityEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelActivityEventArgs[] = L"Windows.Networking.Vpn.IVpnChannelActivityEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("a36c88f2-afdc-4775-855d-d4ac0a35fc55")
                IVpnChannelActivityEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Networking::Vpn::VpnChannelActivityEventType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannelActivityEventArgs = __uuidof(IVpnChannelActivityEventArgs);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelActivityStateChangedArgs[] = L"Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("3d750565-fdc0-4bbe-a23b-45fffc6d97a1")
                IVpnChannelActivityStateChangedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActivityState(
                        ABI::Windows::Networking::Vpn::VpnChannelActivityEventType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannelActivityStateChangedArgs = __uuidof(IVpnChannelActivityStateChangedArgs);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelConfiguration[] = L"Windows.Networking.Vpn.IVpnChannelConfiguration";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("0e2ddca2-2012-4fe4-b179-8c652c6d107e")
                IVpnChannelConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerHostNameList(
                        __FIVectorView_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomField(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannelConfiguration = __uuidof(IVpnChannelConfiguration);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelConfiguration2[] = L"Windows.Networking.Vpn.IVpnChannelConfiguration2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("f30b574c-7824-471c-a118-63dbc93ae4c7")
                IVpnChannelConfiguration2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerUris(
                        __FIVectorView_1_Windows__CFoundation__CUri** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannelConfiguration2 = __uuidof(IVpnChannelConfiguration2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelStatics[] = L"Windows.Networking.Vpn.IVpnChannelStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("88eb062d-e818-4ffd-98a6-363e3736c95d")
                IVpnChannelStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ProcessEventAsync(
                        IInspectable* thirdPartyPlugIn,
                        IInspectable* event
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnChannelStatics = __uuidof(IVpnChannelStatics);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCredential[] = L"Windows.Networking.Vpn.IVpnCredential";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("b7e78af3-a46d-404b-8729-1832522853ac")
                IVpnCredential : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PasskeyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CertificateCredential(
                        ABI::Windows::Security::Cryptography::Certificates::ICertificate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdditionalPin(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OldPasswordCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCredential = __uuidof(IVpnCredential);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomCheckBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomCheckBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomCheckBox[] = L"Windows.Networking.Vpn.IVpnCustomCheckBox";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("43878753-03c5-4e61-93d7-a957714c4282")
                IVpnCustomCheckBox : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_InitialCheckState(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InitialCheckState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Checked(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomCheckBox = __uuidof(IVpnCustomCheckBox);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomComboBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomComboBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomComboBox[] = L"Windows.Networking.Vpn.IVpnCustomComboBox";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9a24158e-dba1-4c6f-8270-dcf3c9761c4c")
                IVpnCustomComboBox : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_OptionsText(
                        __FIVectorView_1_HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OptionsText(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Selected(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomComboBox = __uuidof(IVpnCustomComboBox);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomEditBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomEditBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomEditBox[] = L"Windows.Networking.Vpn.IVpnCustomEditBox";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("3002d9a0-cfbf-4c0b-8f3c-66f503c20b39")
                IVpnCustomEditBox : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DefaultText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NoEcho(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NoEcho(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomEditBox = __uuidof(IVpnCustomEditBox);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomErrorBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomErrorBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomErrorBox[] = L"Windows.Networking.Vpn.IVpnCustomErrorBox";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9ec4efb2-c942-42af-b223-588b48328721")
                IVpnCustomErrorBox : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IVpnCustomErrorBox = __uuidof(IVpnCustomErrorBox);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPrompt[] = L"Windows.Networking.Vpn.IVpnCustomPrompt";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9b2ebe7b-87d5-433c-b4f6-eee6aa68a244")
                IVpnCustomPrompt : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Label(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Compulsory(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Compulsory(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Bordered(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bordered(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPrompt = __uuidof(IVpnCustomPrompt);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptBooleanInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptBooleanInput
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptBooleanInput[] = L"Windows.Networking.Vpn.IVpnCustomPromptBooleanInput";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("c4c9a69e-ff47-4527-9f27-a49292019979")
                IVpnCustomPromptBooleanInput : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_InitialValue(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InitialValue(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Value(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPromptBooleanInput = __uuidof(IVpnCustomPromptBooleanInput);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptElement[] = L"Windows.Networking.Vpn.IVpnCustomPromptElement";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("73bd5638-6f04-404d-93dd-50a44924a38b")
                IVpnCustomPromptElement : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Compulsory(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Compulsory(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Emphasized(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Emphasized(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPromptElement = __uuidof(IVpnCustomPromptElement);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptOptionSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptOptionSelector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptOptionSelector[] = L"Windows.Networking.Vpn.IVpnCustomPromptOptionSelector";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("3b8f34d9-8ec1-4e95-9a4e-7ba64d38f330")
                IVpnCustomPromptOptionSelector : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Options(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SelectedIndex(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPromptOptionSelector = __uuidof(IVpnCustomPromptOptionSelector);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptText
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptText[] = L"Windows.Networking.Vpn.IVpnCustomPromptText";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("3bc8bdee-3a42-49a3-abdd-07b2edea752d")
                IVpnCustomPromptText : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPromptText = __uuidof(IVpnCustomPromptText);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptTextInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptTextInput
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptTextInput[] = L"Windows.Networking.Vpn.IVpnCustomPromptTextInput";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("c9da9c75-913c-47d5-88ba-48fc48930235")
                IVpnCustomPromptTextInput : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PlaceholderText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PlaceholderText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsTextHidden(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsTextHidden(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomPromptTextInput = __uuidof(IVpnCustomPromptTextInput);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomTextBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomTextBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomTextBox[] = L"Windows.Networking.Vpn.IVpnCustomTextBox";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("daa4c3ca-8f23-4d36-91f1-76d937827942")
                IVpnCustomTextBox : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayText(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnCustomTextBox = __uuidof(IVpnCustomTextBox);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameAssignment[] = L"Windows.Networking.Vpn.IVpnDomainNameAssignment";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("4135b141-ccdb-49b5-9401-039a8ae767e9")
                IVpnDomainNameAssignment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DomainNameList(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProxyAutoConfigurationUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyAutoConfigurationUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnDomainNameAssignment = __uuidof(IVpnDomainNameAssignment);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfo[] = L"Windows.Networking.Vpn.IVpnDomainNameInfo";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("ad2eb82f-ea8e-4f7a-843e-1a87e32e1b9a")
                IVpnDomainNameInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DomainName(
                        ABI::Windows::Networking::IHostName* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DomainName(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DomainNameType(
                        ABI::Windows::Networking::Vpn::VpnDomainNameType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DomainNameType(
                        ABI::Windows::Networking::Vpn::VpnDomainNameType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DnsServers(
                        __FIVector_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WebProxyServers(
                        __FIVector_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnDomainNameInfo = __uuidof(IVpnDomainNameInfo);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfo2[] = L"Windows.Networking.Vpn.IVpnDomainNameInfo2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("ab871151-6c53-4828-9883-d886de104407")
                IVpnDomainNameInfo2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WebProxyUris(
                        __FIVector_1_Windows__CFoundation__CUri** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnDomainNameInfo2 = __uuidof(IVpnDomainNameInfo2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfoFactory[] = L"Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("2507bb75-028f-4688-8d3a-c4531df37da8")
                IVpnDomainNameInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVpnDomainNameInfo(
                        HSTRING name,
                        ABI::Windows::Networking::Vpn::VpnDomainNameType nameType,
                        __FIIterable_1_Windows__CNetworking__CHostName* dnsServerList,
                        __FIIterable_1_Windows__CNetworking__CHostName* proxyServerList,
                        ABI::Windows::Networking::Vpn::IVpnDomainNameInfo** domainNameInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnDomainNameInfoFactory = __uuidof(IVpnDomainNameInfoFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnForegroundActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnForegroundActivatedEventArgs[] = L"Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("85b465b0-cadb-4d70-ac92-543a24dc9ebc")
                IVpnForegroundActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProfileName(
                        HSTRING* name
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SharedContext(
                        ABI::Windows::Foundation::Collections::IPropertySet** sharedContext
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActivationOperation(
                        ABI::Windows::Networking::Vpn::IVpnForegroundActivationOperation** activationOperation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnForegroundActivatedEventArgs = __uuidof(IVpnForegroundActivatedEventArgs);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnForegroundActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnForegroundActivationOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnForegroundActivationOperation[] = L"Windows.Networking.Vpn.IVpnForegroundActivationOperation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9e010d57-f17a-4bd5-9b6d-f984f1297d3c")
                IVpnForegroundActivationOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(
                        ABI::Windows::Foundation::Collections::IPropertySet* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnForegroundActivationOperation = __uuidof(IVpnForegroundActivationOperation);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnInterfaceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnInterfaceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnInterfaceId[] = L"Windows.Networking.Vpn.IVpnInterfaceId";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9e2ddca2-1712-4ce4-b179-8c652c6d1011")
                IVpnInterfaceId : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAddressInfo(
                        UINT32* idLength,
                        BYTE** id
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnInterfaceId = __uuidof(IVpnInterfaceId);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnInterfaceIdFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnInterfaceIdFactory[] = L"Windows.Networking.Vpn.IVpnInterfaceIdFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9e2ddca2-1712-4ce4-b179-8c652c6d1000")
                IVpnInterfaceIdFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVpnInterfaceId(
                        UINT32 addressLength,
                        BYTE* address,
                        ABI::Windows::Networking::Vpn::IVpnInterfaceId** vpnInterfaceId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnInterfaceIdFactory = __uuidof(IVpnInterfaceIdFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnManagementAgent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnManagementAgent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnManagementAgent[] = L"Windows.Networking.Vpn.IVpnManagementAgent";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("193696cd-a5c4-4abe-852b-785be4cb3e34")
                IVpnManagementAgent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddProfileFromXmlAsync(
                        HSTRING xml,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddProfileFromObjectAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateProfileFromXmlAsync(
                        HSTRING xml,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateProfileFromObjectAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetProfilesAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteProfileAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectProfileAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectProfileWithPasswordCredentialAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        ABI::Windows::Security::Credentials::IPasswordCredential* passwordCredential,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisconnectProfileAsync(
                        ABI::Windows::Networking::Vpn::IVpnProfile* profile,
                        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnManagementAgent = __uuidof(IVpnManagementAgent);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNamespaceAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceAssignment[] = L"Windows.Networking.Vpn.IVpnNamespaceAssignment";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("d7f7db18-307d-4c0e-bd62-8fa270bbadd6")
                IVpnNamespaceAssignment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_NamespaceList(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NamespaceList(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProxyAutoConfigUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyAutoConfigUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnNamespaceAssignment = __uuidof(IVpnNamespaceAssignment);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNamespaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceInfo[] = L"Windows.Networking.Vpn.IVpnNamespaceInfo";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("30edfb43-444f-44c5-8167-a35a91f1af94")
                IVpnNamespaceInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Namespace(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Namespace(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DnsServers(
                        __FIVector_1_Windows__CNetworking__CHostName* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DnsServers(
                        __FIVector_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WebProxyServers(
                        __FIVector_1_Windows__CNetworking__CHostName* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WebProxyServers(
                        __FIVector_1_Windows__CNetworking__CHostName** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnNamespaceInfo = __uuidof(IVpnNamespaceInfo);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceInfoFactory[] = L"Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("cb3e951a-b0ce-442b-acbb-5f99b202c31c")
                IVpnNamespaceInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVpnNamespaceInfo(
                        HSTRING name,
                        __FIVector_1_Windows__CNetworking__CHostName* dnsServerList,
                        __FIVector_1_Windows__CNetworking__CHostName* proxyServerList,
                        ABI::Windows::Networking::Vpn::IVpnNamespaceInfo** namespaceInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnNamespaceInfoFactory = __uuidof(IVpnNamespaceInfoFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNativeProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNativeProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNativeProfile[] = L"Windows.Networking.Vpn.IVpnNativeProfile";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("a4aee29e-6417-4333-9842-f0a66db69802")
                IVpnNativeProfile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Servers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoutingPolicyType(
                        ABI::Windows::Networking::Vpn::VpnRoutingPolicyType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoutingPolicyType(
                        ABI::Windows::Networking::Vpn::VpnRoutingPolicyType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NativeProtocolType(
                        ABI::Windows::Networking::Vpn::VpnNativeProtocolType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NativeProtocolType(
                        ABI::Windows::Networking::Vpn::VpnNativeProtocolType value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserAuthenticationMethod(
                        ABI::Windows::Networking::Vpn::VpnAuthenticationMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UserAuthenticationMethod(
                        ABI::Windows::Networking::Vpn::VpnAuthenticationMethod value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TunnelAuthenticationMethod(
                        ABI::Windows::Networking::Vpn::VpnAuthenticationMethod* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TunnelAuthenticationMethod(
                        ABI::Windows::Networking::Vpn::VpnAuthenticationMethod value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EapConfiguration(
                        HSTRING* Value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EapConfiguration(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnNativeProfile = __uuidof(IVpnNativeProfile);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNativeProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNativeProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNativeProfile2[] = L"Windows.Networking.Vpn.IVpnNativeProfile2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("0fec2467-cdb5-4ac7-b5a3-0afb5ec47682")
                IVpnNativeProfile2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequireVpnClientAppUI(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequireVpnClientAppUI(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        ABI::Windows::Networking::Vpn::VpnManagementConnectionStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnNativeProfile2 = __uuidof(IVpnNativeProfile2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer[] = L"Windows.Networking.Vpn.IVpnPacketBuffer";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("c2f891fc-4d5c-4a63-b70d-4e307eacce55")
                IVpnPacketBuffer : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Buffer(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Status(
                        ABI::Windows::Networking::Vpn::VpnPacketBufferStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::Vpn::VpnPacketBufferStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransportAffinity(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportAffinity(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBuffer = __uuidof(IVpnPacketBuffer);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer2[] = L"Windows.Networking.Vpn.IVpnPacketBuffer2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("665e91f0-8805-4bf5-a619-2e84882e6b4f")
                IVpnPacketBuffer2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        ABI::Windows::Networking::Vpn::IVpnAppId** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBuffer2 = __uuidof(IVpnPacketBuffer2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer3[] = L"Windows.Networking.Vpn.IVpnPacketBuffer3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("e256072f-107b-4c40-b127-5bc53e0ad960")
                IVpnPacketBuffer3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_TransportContext(
                        IInspectable* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportContext(
                        IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBuffer3 = __uuidof(IVpnPacketBuffer3);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferFactory[] = L"Windows.Networking.Vpn.IVpnPacketBufferFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9e2ddca2-1712-4ce4-b179-8c652c6d9999")
                IVpnPacketBufferFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVpnPacketBuffer(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* parentBuffer,
                        UINT32 offset,
                        UINT32 length,
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** vpnPacketBuffer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBufferFactory = __uuidof(IVpnPacketBufferFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferList[] = L"Windows.Networking.Vpn.IVpnPacketBufferList";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("c2f891fc-4d5c-4a63-b70d-4e307eacce77")
                IVpnPacketBufferList : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Append(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddAtBegin(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveAtEnd(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveAtBegin(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Status(
                        ABI::Windows::Networking::Vpn::VpnPacketBufferStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Networking::Vpn::VpnPacketBufferStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Size(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBufferList = __uuidof(IVpnPacketBufferList);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferList2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferList2[] = L"Windows.Networking.Vpn.IVpnPacketBufferList2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("3e7acfe5-ea1e-482a-8d98-c065f57d89ea")
                IVpnPacketBufferList2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AddLeadingPacket(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveLeadingPacket(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddTrailingPacket(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* nextVpnPacketBuffer
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveTrailingPacket(
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** nextVpnPacketBuffer
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPacketBufferList2 = __uuidof(IVpnPacketBufferList2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPickedCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPickedCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPickedCredential[] = L"Windows.Networking.Vpn.IVpnPickedCredential";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9a793ac7-8854-4e52-ad97-24dd9a842bce")
                IVpnPickedCredential : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PasskeyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AdditionalPin(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OldPasswordCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPickedCredential = __uuidof(IVpnPickedCredential);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugIn
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugIn[] = L"Windows.Networking.Vpn.IVpnPlugIn";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("ceb78d07-d0a8-4703-a091-c8c2c0915bc4")
                IVpnPlugIn : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Connect(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Disconnect(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetKeepAlivePayload(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel,
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer** keepAlivePacket
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Encapsulate(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel,
                        ABI::Windows::Networking::Vpn::IVpnPacketBufferList* packets,
                        ABI::Windows::Networking::Vpn::IVpnPacketBufferList* encapulatedPackets
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Decapsulate(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel,
                        ABI::Windows::Networking::Vpn::IVpnPacketBuffer* encapBuffer,
                        ABI::Windows::Networking::Vpn::IVpnPacketBufferList* decapsulatedPackets,
                        ABI::Windows::Networking::Vpn::IVpnPacketBufferList* controlPacketsToSend
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPlugIn = __uuidof(IVpnPlugIn);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInProfile[] = L"Windows.Networking.Vpn.IVpnPlugInProfile";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("0edf0da4-4f00-4589-8d7b-4bf988f6542c")
                IVpnPlugInProfile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServerUris(
                        __FIVector_1_Windows__CFoundation__CUri** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CustomConfiguration(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CustomConfiguration(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VpnPluginPackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_VpnPluginPackageFamilyName(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPlugInProfile = __uuidof(IVpnPlugInProfile);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInProfile2[] = L"Windows.Networking.Vpn.IVpnPlugInProfile2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("611c4892-cf94-4ad6-ba99-00f4ff34565e")
                IVpnPlugInProfile2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequireVpnClientAppUI(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequireVpnClientAppUI(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConnectionStatus(
                        ABI::Windows::Networking::Vpn::VpnManagementConnectionStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPlugInProfile2 = __uuidof(IVpnPlugInProfile2);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInReconnectTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInReconnectTransport[] = L"Windows.Networking.Vpn.IVpnPlugInReconnectTransport";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("9d5a1092-bb46-4d34-9d88-f217893076f4")
                IVpnPlugInReconnectTransport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReconnectTransport(
                        ABI::Windows::Networking::Vpn::IVpnChannel* channel,
                        IInspectable* context
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnPlugInReconnectTransport = __uuidof(IVpnPlugInReconnectTransport);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnProfile[] = L"Windows.Networking.Vpn.IVpnProfile";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("7875b751-b0d7-43db-8a93-d3fe2479e56a")
                IVpnProfile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProfileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProfileName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppTriggers(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Routes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DomainNameInfoList(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrafficFilters(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RememberCredentials(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RememberCredentials(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AlwaysOn(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AlwaysOn(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnProfile = __uuidof(IVpnProfile);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRoute[] = L"Windows.Networking.Vpn.IVpnRoute";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("b5731b83-0969-4699-938e-7776db29cfb3")
                IVpnRoute : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Address(
                        ABI::Windows::Networking::IHostName* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Address(
                        ABI::Windows::Networking::IHostName** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrefixSize(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrefixSize(
                        BYTE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnRoute = __uuidof(IVpnRoute);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRouteAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnRouteAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRouteAssignment[] = L"Windows.Networking.Vpn.IVpnRouteAssignment";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("db64de22-ce39-4a76-9550-f61039f80e48")
                IVpnRouteAssignment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Ipv4InclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ipv6InclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ipv4InclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ipv6InclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ipv4ExclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ipv6ExclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ipv4ExclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ipv6ExclusionRoutes(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExcludeLocalSubnets(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExcludeLocalSubnets(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnRouteAssignment = __uuidof(IVpnRouteAssignment);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRouteFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRouteFactory[] = L"Windows.Networking.Vpn.IVpnRouteFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("bdeab5ff-45cf-4b99-83fb-db3bc2672b02")
                IVpnRouteFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVpnRoute(
                        ABI::Windows::Networking::IHostName* address,
                        BYTE prefixSize,
                        ABI::Windows::Networking::Vpn::IVpnRoute** route
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnRouteFactory = __uuidof(IVpnRouteFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnSystemHealth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnSystemHealth
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnSystemHealth[] = L"Windows.Networking.Vpn.IVpnSystemHealth";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("99a8f8af-c0ee-4e75-817a-f231aee5123d")
                IVpnSystemHealth : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StatementOfHealth(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnSystemHealth = __uuidof(IVpnSystemHealth);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilter[] = L"Windows.Networking.Vpn.IVpnTrafficFilter";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("2f691b60-6c9f-47f5-ac36-bb1b042e2c50")
                IVpnTrafficFilter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        ABI::Windows::Networking::Vpn::IVpnAppId** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AppId(
                        ABI::Windows::Networking::Vpn::IVpnAppId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppClaims(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Protocol(
                        ABI::Windows::Networking::Vpn::VpnIPProtocol* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Protocol(
                        ABI::Windows::Networking::Vpn::VpnIPProtocol value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalPortRanges(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemotePortRanges(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalAddressRanges(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteAddressRanges(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoutingPolicyType(
                        ABI::Windows::Networking::Vpn::VpnRoutingPolicyType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoutingPolicyType(
                        ABI::Windows::Networking::Vpn::VpnRoutingPolicyType value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnTrafficFilter = __uuidof(IVpnTrafficFilter);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilterAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilterAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilterAssignment[] = L"Windows.Networking.Vpn.IVpnTrafficFilterAssignment";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("56ccd45c-e664-471e-89cd-601603b9e0f3")
                IVpnTrafficFilterAssignment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TrafficFilterList(
                        __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowOutbound(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowOutbound(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowInbound(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowInbound(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnTrafficFilterAssignment = __uuidof(IVpnTrafficFilterAssignment);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilterFactory[] = L"Windows.Networking.Vpn.IVpnTrafficFilterFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace Vpn {
                MIDL_INTERFACE("480d41d5-7f99-474c-86ee-96df168318f1")
                IVpnTrafficFilterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Networking::Vpn::IVpnAppId* appId,
                        ABI::Windows::Networking::Vpn::IVpnTrafficFilter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVpnTrafficFilterFactory = __uuidof(IVpnTrafficFilterFactory);
            } /* Vpn */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnAppId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnAppIdFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnAppId ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnAppId_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnAppId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnAppId[] = L"Windows.Networking.Vpn.VpnAppId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Vpn.IVpnChannelStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannel ** Default Interface **
 *    Windows.Networking.Vpn.IVpnChannel2
 *    Windows.Networking.Vpn.IVpnChannel4
 *    Windows.Networking.Vpn.IVpnChannel5
 *    Windows.Networking.Vpn.IVpnChannel6
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannel_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannel[] = L"Windows.Networking.Vpn.VpnChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelActivityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelActivityEventArgs ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelActivityEventArgs[] = L"Windows.Networking.Vpn.VpnChannelActivityEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs[] = L"Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelConfiguration ** Default Interface **
 *    Windows.Networking.Vpn.IVpnChannelConfiguration2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelConfiguration[] = L"Windows.Networking.Vpn.VpnChannelConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCredential ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCredential_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCredential[] = L"Windows.Networking.Vpn.VpnCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomCheckBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomCheckBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomCheckBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomCheckBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomCheckBox[] = L"Windows.Networking.Vpn.VpnCustomCheckBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomComboBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomComboBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomComboBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomComboBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomComboBox[] = L"Windows.Networking.Vpn.VpnCustomComboBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomEditBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomEditBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomEditBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomEditBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomEditBox[] = L"Windows.Networking.Vpn.VpnCustomEditBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomErrorBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomErrorBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomErrorBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomErrorBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomErrorBox[] = L"Windows.Networking.Vpn.VpnCustomErrorBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptBooleanInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptBooleanInput ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptBooleanInput_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptBooleanInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptBooleanInput[] = L"Windows.Networking.Vpn.VpnCustomPromptBooleanInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptOptionSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptOptionSelector ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptOptionSelector_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptOptionSelector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptOptionSelector[] = L"Windows.Networking.Vpn.VpnCustomPromptOptionSelector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptText ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptText_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptText[] = L"Windows.Networking.Vpn.VpnCustomPromptText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptTextInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptTextInput ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptTextInput_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptTextInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptTextInput[] = L"Windows.Networking.Vpn.VpnCustomPromptTextInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomTextBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomTextBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomTextBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomTextBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomTextBox[] = L"Windows.Networking.Vpn.VpnCustomTextBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnDomainNameAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnDomainNameAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnDomainNameAssignment[] = L"Windows.Networking.Vpn.VpnDomainNameAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnDomainNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnDomainNameInfoFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnDomainNameInfo ** Default Interface **
 *    Windows.Networking.Vpn.IVpnDomainNameInfo2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameInfo_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnDomainNameInfo[] = L"Windows.Networking.Vpn.VpnDomainNameInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnForegroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs[] = L"Windows.Networking.Vpn.VpnForegroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Networking.Vpn.VpnForegroundActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnForegroundActivationOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivationOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnForegroundActivationOperation[] = L"Windows.Networking.Vpn.VpnForegroundActivationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Networking.Vpn.VpnInterfaceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnInterfaceIdFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnInterfaceId ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnInterfaceId_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnInterfaceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnInterfaceId[] = L"Windows.Networking.Vpn.VpnInterfaceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnManagementAgent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnManagementAgent ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnManagementAgent_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnManagementAgent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnManagementAgent[] = L"Windows.Networking.Vpn.VpnManagementAgent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNamespaceAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNamespaceAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNamespaceAssignment[] = L"Windows.Networking.Vpn.VpnNamespaceAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNamespaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnNamespaceInfoFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNamespaceInfo ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNamespaceInfo[] = L"Windows.Networking.Vpn.VpnNamespaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNativeProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNativeProfile ** Default Interface **
 *    Windows.Networking.Vpn.IVpnProfile
 *    Windows.Networking.Vpn.IVpnNativeProfile2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNativeProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNativeProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNativeProfile[] = L"Windows.Networking.Vpn.VpnNativeProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPacketBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnPacketBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPacketBuffer ** Default Interface **
 *    Windows.Networking.Vpn.IVpnPacketBuffer2
 *    Windows.Networking.Vpn.IVpnPacketBuffer3
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBuffer_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPacketBuffer[] = L"Windows.Networking.Vpn.VpnPacketBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPacketBufferList ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBufferList_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBufferList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPacketBufferList[] = L"Windows.Networking.Vpn.VpnPacketBufferList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPickedCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPickedCredential ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPickedCredential_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPickedCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPickedCredential[] = L"Windows.Networking.Vpn.VpnPickedCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPlugInProfile ** Default Interface **
 *    Windows.Networking.Vpn.IVpnProfile
 *    Windows.Networking.Vpn.IVpnPlugInProfile2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPlugInProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPlugInProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPlugInProfile[] = L"Windows.Networking.Vpn.VpnPlugInProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnRouteFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnRoute ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnRoute_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnRoute[] = L"Windows.Networking.Vpn.VpnRoute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnRouteAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnRouteAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnRouteAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnRouteAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnRouteAssignment[] = L"Windows.Networking.Vpn.VpnRouteAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnSystemHealth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnSystemHealth ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnSystemHealth_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnSystemHealth_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnSystemHealth[] = L"Windows.Networking.Vpn.VpnSystemHealth";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnTrafficFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnTrafficFilterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnTrafficFilter ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnTrafficFilter[] = L"Windows.Networking.Vpn.VpnTrafficFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnTrafficFilterAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnTrafficFilterAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilterAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilterAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnTrafficFilterAssignment[] = L"Windows.Networking.Vpn.VpnTrafficFilterAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4 __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5 __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6 __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3 __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2 __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory;

#endif // ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CIVpnProfile** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential* This,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnCredential_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementErrorStatus __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementErrorStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementErrorStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* This,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1_IInspectable __FIIterator_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_IInspectable;

typedef struct __FIIterator_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_IInspectable* This,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_IInspectable* This,
        UINT32 itemsLength,
        IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_IInspectableVtbl;

interface __FIIterator_1_IInspectable
{
    CONST_VTBL struct __FIIterator_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1_IInspectable __FIIterable_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_IInspectable;

typedef struct __FIIterable_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_IInspectable* This,
        __FIIterator_1_IInspectable** result);

    END_INTERFACE
} __FIIterable_1_IInspectableVtbl;

interface __FIIterable_1_IInspectable
{
    CONST_VTBL struct __FIIterable_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CUri __FIIterator_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CUri;

typedef struct __FIIterator_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CUriVtbl;

interface __FIIterator_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CUri_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CUri __FIIterable_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CUri;

typedef struct __FIIterable_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CUri* This,
        __FIIterator_1_Windows__CFoundation__CUri** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CUriVtbl;

interface __FIIterable_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CUri_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CIHostName __x_ABI_CWindows_CNetworking_CIHostName;

#endif // ____x_ABI_CWindows_CNetworking_CIHostName_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CHostName __FIIterator_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CHostName;

typedef struct __FIIterator_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CHostName* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CHostNameVtbl;

interface __FIIterator_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CHostName_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CHostName_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CHostName __FIIterable_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CHostName;

typedef struct __FIIterable_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CHostName* This,
        __FIIterator_1_Windows__CNetworking__CHostName** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CHostNameVtbl;

interface __FIIterable_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CHostName_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnAppId** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnPacketBuffer** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnPacketBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnRouteVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnRoute** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnRouteVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

typedef struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl;

interface __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

typedef struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __FIIterator_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl;

interface __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CUri __FIVectorView_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CUri;

typedef struct __FIVectorView_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CUriVtbl;

interface __FIVectorView_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CHostName __FIVectorView_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CHostName;

typedef struct __FIVectorView_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIHostName** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CHostName* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CHostNameVtbl;

interface __FIVectorView_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CHostName_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRouteVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

typedef struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl;

interface __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CFoundation__CUri __FIVector_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CFoundation__CUri;

typedef struct __FIVector_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CFoundation__CUri* This,
        __FIVectorView_1_Windows__CFoundation__CUri** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items);

    END_INTERFACE
} __FIVector_1_Windows__CFoundation__CUriVtbl;

interface __FIVector_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVector_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CFoundation__CUri_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CFoundation__CUri_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CUri_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CUri_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CFoundation__CUri_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CFoundation__CUri_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CFoundation__CUri_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CFoundation__CUri_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CHostName __FIVector_1_Windows__CNetworking__CHostName;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CHostName;

typedef struct __FIVector_1_Windows__CNetworking__CHostNameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CHostName* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CHostName* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CHostName* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CHostName* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CHostName* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIHostName** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CHostName* This,
        __FIVectorView_1_Windows__CNetworking__CHostName** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CHostName* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CHostName* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CHostName* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CIHostName** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CHostNameVtbl;

interface __FIVector_1_Windows__CNetworking__CHostName
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CHostNameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CHostName_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CHostName_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CHostName_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CHostName_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CHostName_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CHostName_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CHostName_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CHostName_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CHostName_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CHostName_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CHostName_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CHostName_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CHostName_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CHostName_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CHostName_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CHostName_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CHostName_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CHostName_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CHostName_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CVpn__CVpnAppId;

typedef struct __FIVector_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CVpnAppId** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CVpn__CVpnAppId* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl;

interface __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CVpn__CVpnAppIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CVpn__CVpnAppId_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo;

typedef struct __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl;

interface __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo;

typedef struct __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl;

interface __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CVpn__CVpnRoute;

typedef struct __FIVector_1_Windows__CNetworking__CVpn__CVpnRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CVpnRoute** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CVpn__CVpnRouteVtbl;

interface __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CVpn__CVpnRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CVpn__CVpnRoute_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter;

typedef struct __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl;

interface __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* sender,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* sender,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus;

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelActivityEventType __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelActivityEventType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDataPathType __x_ABI_CWindows_CNetworking_CVpn_CVpnDataPathType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnIPProtocol __x_ABI_CWindows_CNetworking_CVpn_CVpnIPProtocol;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementConnectionStatus __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementConnectionStatus;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnNativeProtocolType __x_ABI_CWindows_CNetworking_CVpn_CVpnNativeProtocolType;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus;

typedef enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType;

/*
 *
 * Struct Windows.Networking.Vpn.VpnAppIdType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType
{
    VpnAppIdType_PackageFamilyName = 0,
    VpnAppIdType_FullyQualifiedBinaryName = 1,
    VpnAppIdType_FilePath = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnAuthenticationMethod
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod
{
    VpnAuthenticationMethod_Mschapv2 = 0,
    VpnAuthenticationMethod_Eap = 1,
    VpnAuthenticationMethod_Certificate = 2,
    VpnAuthenticationMethod_PresharedKey = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnChannelActivityEventType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelActivityEventType
{
    VpnChannelActivityEventType_Idle = 0,
    VpnChannelActivityEventType_Active = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnChannelRequestCredentialsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelRequestCredentialsOptions
{
    VpnChannelRequestCredentialsOptions_None = 0,
    VpnChannelRequestCredentialsOptions_Retrying = 0x1,
    VpnChannelRequestCredentialsOptions_UseForSingleSignIn = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnCredentialType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType
{
    VpnCredentialType_UsernamePassword = 0,
    VpnCredentialType_UsernameOtpPin = 1,
    VpnCredentialType_UsernamePasswordAndPin = 2,
    VpnCredentialType_UsernamePasswordChange = 3,
    VpnCredentialType_SmartCard = 4,
    VpnCredentialType_ProtectedCertificate = 5,
    VpnCredentialType_UnProtectedCertificate = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnDataPathType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDataPathType
{
    VpnDataPathType_Send = 0,
    VpnDataPathType_Receive = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnDomainNameType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType
{
    VpnDomainNameType_Suffix = 0,
    VpnDomainNameType_FullyQualified = 1,
    VpnDomainNameType_Reserved = 65535,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnIPProtocol
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnIPProtocol
{
    VpnIPProtocol_None = 0,
    VpnIPProtocol_Tcp = 6,
    VpnIPProtocol_Udp = 17,
    VpnIPProtocol_Icmp = 1,
    VpnIPProtocol_Ipv6Icmp = 58,
    VpnIPProtocol_Igmp = 2,
    VpnIPProtocol_Pgm = 113,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnManagementConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementConnectionStatus
{
    VpnManagementConnectionStatus_Disconnected = 0,
    VpnManagementConnectionStatus_Disconnecting = 1,
    VpnManagementConnectionStatus_Connected = 2,
    VpnManagementConnectionStatus_Connecting = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnManagementErrorStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementErrorStatus
{
    VpnManagementErrorStatus_Ok = 0,
    VpnManagementErrorStatus_Other = 1,
    VpnManagementErrorStatus_InvalidXmlSyntax = 2,
    VpnManagementErrorStatus_ProfileNameTooLong = 3,
    VpnManagementErrorStatus_ProfileInvalidAppId = 4,
    VpnManagementErrorStatus_AccessDenied = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_CannotFindProfile = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_AlreadyDisconnecting = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_AlreadyConnected = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_GeneralAuthenticationFailure = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_EapFailure = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_SmartCardFailure = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_CertificateFailure = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_ServerConfiguration = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_NoConnection = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_ServerConnection = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_UserNamePassword = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_DnsNotResolvable = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    VpnManagementErrorStatus_InvalidIP = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnNativeProtocolType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnNativeProtocolType
{
    VpnNativeProtocolType_Pptp = 0,
    VpnNativeProtocolType_L2tp = 1,
    VpnNativeProtocolType_IpsecIkev2 = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnPacketBufferStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus
{
    VpnPacketBufferStatus_Ok = 0,
    VpnPacketBufferStatus_InvalidBufferSize = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.Vpn.VpnRoutingPolicyType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType
{
    VpnRoutingPolicyType_SplitRouting = 0,
    VpnRoutingPolicyType_ForceAllTrafficOverVpn = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnAppId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnAppId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnAppId[] = L"Windows.Networking.Vpn.IVpnAppId";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Value)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_put_Value(This, value) \
    ((This)->lpVtbl->put_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnAppIdFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnAppId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnAppIdFactory[] = L"Windows.Networking.Vpn.IVpnAppIdFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAppIdType type,
        HSTRING value,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_Create(This, type, value, result) \
    ((This)->lpVtbl->Create(This, type, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnAppIdFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel[] = L"Windows.Networking.Vpn.IVpnChannel";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AssociateTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        IInspectable* mainOuterTunnelTransport,
        IInspectable* optionalOuterTunnelTransport);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* vpnInterfaceId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* routeScope,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* namespaceScope,
        UINT32 mtuSize,
        UINT32 maxFrameSize,
        boolean optimizeForLowCostNetwork,
        IInspectable* mainOuterTunnelTransport,
        IInspectable* optionalOuterTunnelTransport);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This);
    HRESULT (STDMETHODCALLTYPE* RequestCredentials)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType credType,
        boolean isRetry,
        boolean isSingleSignOnCredential,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential** credential);
    HRESULT (STDMETHODCALLTYPE* RequestVpnPacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDataPathType type,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** vpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* LogDiagnosticMessage)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        HSTRING message);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* add_ActivityChange)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ActivityChange)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* put_PlugInContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* get_PlugInContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemHealth)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth** value);
    HRESULT (STDMETHODCALLTYPE* RequestCustomPrompt)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPrompt* customPrompt);
    HRESULT (STDMETHODCALLTYPE* SetErrorMessage)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        HSTRING message);
    HRESULT (STDMETHODCALLTYPE* SetAllowedSslTlsVersions)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* This,
        IInspectable* tunnelTransport,
        boolean useTls12);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_AssociateTransport(This, mainOuterTunnelTransport, optionalOuterTunnelTransport) \
    ((This)->lpVtbl->AssociateTransport(This, mainOuterTunnelTransport, optionalOuterTunnelTransport))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_Start(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, routeScope, namespaceScope, mtuSize, maxFrameSize, optimizeForLowCostNetwork, mainOuterTunnelTransport, optionalOuterTunnelTransport) \
    ((This)->lpVtbl->Start(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, routeScope, namespaceScope, mtuSize, maxFrameSize, optimizeForLowCostNetwork, mainOuterTunnelTransport, optionalOuterTunnelTransport))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_RequestCredentials(This, credType, isRetry, isSingleSignOnCredential, certificate, credential) \
    ((This)->lpVtbl->RequestCredentials(This, credType, isRetry, isSingleSignOnCredential, certificate, credential))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_RequestVpnPacketBuffer(This, type, vpnPacketBuffer) \
    ((This)->lpVtbl->RequestVpnPacketBuffer(This, type, vpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_LogDiagnosticMessage(This, message) \
    ((This)->lpVtbl->LogDiagnosticMessage(This, message))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_get_Configuration(This, value) \
    ((This)->lpVtbl->get_Configuration(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_add_ActivityChange(This, handler, token) \
    ((This)->lpVtbl->add_ActivityChange(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_remove_ActivityChange(This, token) \
    ((This)->lpVtbl->remove_ActivityChange(This, token))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_put_PlugInContext(This, value) \
    ((This)->lpVtbl->put_PlugInContext(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_get_PlugInContext(This, value) \
    ((This)->lpVtbl->get_PlugInContext(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_get_SystemHealth(This, value) \
    ((This)->lpVtbl->get_SystemHealth(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_RequestCustomPrompt(This, customPrompt) \
    ((This)->lpVtbl->RequestCustomPrompt(This, customPrompt))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_SetErrorMessage(This, message) \
    ((This)->lpVtbl->SetErrorMessage(This, message))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_SetAllowedSslTlsVersions(This, tunnelTransport, useTls12) \
    ((This)->lpVtbl->SetAllowedSslTlsVersions(This, tunnelTransport, useTls12))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel2[] = L"Windows.Networking.Vpn.IVpnChannel2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartWithMainTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* vpnInterfaceId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* assignedRoutes,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* assignedDomainName,
        UINT32 mtuSize,
        UINT32 maxFrameSize,
        boolean Reserved,
        IInspectable* mainOuterTunnelTransport);
    HRESULT (STDMETHODCALLTYPE* StartExistingTransports)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv4list,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIPv6list,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* vpnInterfaceId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* assignedRoutes,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* assignedDomainName,
        UINT32 mtuSize,
        UINT32 maxFrameSize,
        boolean Reserved);
    HRESULT (STDMETHODCALLTYPE* add_ActivityStateChange)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __FITypedEventHandler_2_Windows__CNetworking__CVpn__CVpnChannel_Windows__CNetworking__CVpn__CVpnChannelActivityStateChangedArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ActivityStateChange)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetVpnSendPacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** vpnSendPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* GetVpnReceivePacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** vpnReceivePacketBuffer);
    HRESULT (STDMETHODCALLTYPE* RequestCustomPromptAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __FIVectorView_1_Windows__CNetworking__CVpn__CIVpnCustomPromptElement* customPromptElement,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* RequestCredentialsWithCertificateAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType credType,
        UINT32 credOptions,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* certificate,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential);
    HRESULT (STDMETHODCALLTYPE* RequestCredentialsWithOptionsAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType credType,
        UINT32 credOptions,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential);
    HRESULT (STDMETHODCALLTYPE* RequestCredentialsSimpleAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnCredentialType credType,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnCredential** credential);
    HRESULT (STDMETHODCALLTYPE* TerminateConnection)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        HSTRING message);
    HRESULT (STDMETHODCALLTYPE* StartWithTrafficFilter)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2* This,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIpv4List,
        __FIVectorView_1_Windows__CNetworking__CHostName* assignedClientIpv6List,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* vpnInterfaceId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* assignedRoutes,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* assignedNamespace,
        UINT32 mtuSize,
        UINT32 maxFrameSize,
        boolean reserved,
        IInspectable* mainOuterTunnelTransport,
        IInspectable* optionalOuterTunnelTransport,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* assignedTrafficFilters);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_StartWithMainTransport(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, assignedRoutes, assignedDomainName, mtuSize, maxFrameSize, Reserved, mainOuterTunnelTransport) \
    ((This)->lpVtbl->StartWithMainTransport(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, assignedRoutes, assignedDomainName, mtuSize, maxFrameSize, Reserved, mainOuterTunnelTransport))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_StartExistingTransports(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, assignedRoutes, assignedDomainName, mtuSize, maxFrameSize, Reserved) \
    ((This)->lpVtbl->StartExistingTransports(This, assignedClientIPv4list, assignedClientIPv6list, vpnInterfaceId, assignedRoutes, assignedDomainName, mtuSize, maxFrameSize, Reserved))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_add_ActivityStateChange(This, handler, token) \
    ((This)->lpVtbl->add_ActivityStateChange(This, handler, token))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_remove_ActivityStateChange(This, token) \
    ((This)->lpVtbl->remove_ActivityStateChange(This, token))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_GetVpnSendPacketBuffer(This, vpnSendPacketBuffer) \
    ((This)->lpVtbl->GetVpnSendPacketBuffer(This, vpnSendPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_GetVpnReceivePacketBuffer(This, vpnReceivePacketBuffer) \
    ((This)->lpVtbl->GetVpnReceivePacketBuffer(This, vpnReceivePacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_RequestCustomPromptAsync(This, customPromptElement, action) \
    ((This)->lpVtbl->RequestCustomPromptAsync(This, customPromptElement, action))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_RequestCredentialsWithCertificateAsync(This, credType, credOptions, certificate, credential) \
    ((This)->lpVtbl->RequestCredentialsWithCertificateAsync(This, credType, credOptions, certificate, credential))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_RequestCredentialsWithOptionsAsync(This, credType, credOptions, credential) \
    ((This)->lpVtbl->RequestCredentialsWithOptionsAsync(This, credType, credOptions, credential))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_RequestCredentialsSimpleAsync(This, credType, credential) \
    ((This)->lpVtbl->RequestCredentialsSimpleAsync(This, credType, credential))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_TerminateConnection(This, message) \
    ((This)->lpVtbl->TerminateConnection(This, message))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_StartWithTrafficFilter(This, assignedClientIpv4List, assignedClientIpv6List, vpnInterfaceId, assignedRoutes, assignedNamespace, mtuSize, maxFrameSize, reserved, mainOuterTunnelTransport, optionalOuterTunnelTransport, assignedTrafficFilters) \
    ((This)->lpVtbl->StartWithTrafficFilter(This, assignedClientIpv4List, assignedClientIpv6List, vpnInterfaceId, assignedRoutes, assignedNamespace, mtuSize, maxFrameSize, reserved, mainOuterTunnelTransport, optionalOuterTunnelTransport, assignedTrafficFilters))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel4[] = L"Windows.Networking.Vpn.IVpnChannel4";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddAndAssociateTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        IInspectable* transport,
        IInspectable* context);
    HRESULT (STDMETHODCALLTYPE* StartWithMultipleTransports)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        __FIIterable_1_Windows__CNetworking__CHostName* assignedClientIpv4Addresses,
        __FIIterable_1_Windows__CNetworking__CHostName* assignedClientIpv6Addresses,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* vpninterfaceId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* assignedRoutes,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* assignedNamespace,
        UINT32 mtuSize,
        UINT32 maxFrameSize,
        boolean reserved,
        __FIIterable_1_IInspectable* transports,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* assignedTrafficFilters);
    HRESULT (STDMETHODCALLTYPE* ReplaceAndAssociateTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        IInspectable* transport,
        IInspectable* context);
    HRESULT (STDMETHODCALLTYPE* StartReconnectingTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        IInspectable* transport,
        IInspectable* context);
    HRESULT (STDMETHODCALLTYPE* GetSlotTypeForTransportContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        IInspectable* context,
        enum __x_ABI_CWindows_CNetworking_CSockets_CControlChannelTriggerStatus* slotType);
    HRESULT (STDMETHODCALLTYPE* get_CurrentRequestTransportContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4* This,
        IInspectable** context);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_AddAndAssociateTransport(This, transport, context) \
    ((This)->lpVtbl->AddAndAssociateTransport(This, transport, context))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_StartWithMultipleTransports(This, assignedClientIpv4Addresses, assignedClientIpv6Addresses, vpninterfaceId, assignedRoutes, assignedNamespace, mtuSize, maxFrameSize, reserved, transports, assignedTrafficFilters) \
    ((This)->lpVtbl->StartWithMultipleTransports(This, assignedClientIpv4Addresses, assignedClientIpv6Addresses, vpninterfaceId, assignedRoutes, assignedNamespace, mtuSize, maxFrameSize, reserved, transports, assignedTrafficFilters))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_ReplaceAndAssociateTransport(This, transport, context) \
    ((This)->lpVtbl->ReplaceAndAssociateTransport(This, transport, context))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_StartReconnectingTransport(This, transport, context) \
    ((This)->lpVtbl->StartReconnectingTransport(This, transport, context))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_GetSlotTypeForTransportContext(This, context, slotType) \
    ((This)->lpVtbl->GetSlotTypeForTransportContext(This, context, slotType))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_get_CurrentRequestTransportContext(This, context) \
    ((This)->lpVtbl->get_CurrentRequestTransportContext(This, context))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel5[] = L"Windows.Networking.Vpn.IVpnChannel5";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AppendVpnReceivePacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* decapsulatedPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* AppendVpnSendPacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* encapsulatedPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* FlushVpnReceivePacketBuffers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This);
    HRESULT (STDMETHODCALLTYPE* FlushVpnSendPacketBuffers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_AppendVpnReceivePacketBuffer(This, decapsulatedPacketBuffer) \
    ((This)->lpVtbl->AppendVpnReceivePacketBuffer(This, decapsulatedPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_AppendVpnSendPacketBuffer(This, encapsulatedPacketBuffer) \
    ((This)->lpVtbl->AppendVpnSendPacketBuffer(This, encapsulatedPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FlushVpnReceivePacketBuffers(This) \
    ((This)->lpVtbl->FlushVpnReceivePacketBuffers(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_FlushVpnSendPacketBuffers(This) \
    ((This)->lpVtbl->FlushVpnSendPacketBuffers(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannel6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannel6[] = L"Windows.Networking.Vpn.IVpnChannel6";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ActivateForeground)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6* This,
        HSTRING packageRelativeAppId,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* sharedContext,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_ActivateForeground(This, packageRelativeAppId, sharedContext, result) \
    ((This)->lpVtbl->ActivateForeground(This, packageRelativeAppId, sharedContext, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelActivityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelActivityEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelActivityEventArgs[] = L"Windows.Networking.Vpn.IVpnChannelActivityEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelActivityEventType* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelActivityStateChangedArgs[] = L"Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityState)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnChannelActivityEventType* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_get_ActivityState(This, value) \
    ((This)->lpVtbl->get_ActivityState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelActivityStateChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelConfiguration[] = L"Windows.Networking.Vpn.IVpnChannelConfiguration";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerServiceName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ServerHostNameList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        __FIVectorView_1_Windows__CNetworking__CHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_CustomField)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfigurationVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_get_ServerServiceName(This, value) \
    ((This)->lpVtbl->get_ServerServiceName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_get_ServerHostNameList(This, value) \
    ((This)->lpVtbl->get_ServerHostNameList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_get_CustomField(This, value) \
    ((This)->lpVtbl->get_CustomField(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnChannelConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelConfiguration2[] = L"Windows.Networking.Vpn.IVpnChannelConfiguration2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerUris)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_get_ServerUris(This, value) \
    ((This)->lpVtbl->get_ServerUris(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnChannelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnChannelStatics[] = L"Windows.Networking.Vpn.IVpnChannelStatics";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ProcessEventAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics* This,
        IInspectable* thirdPartyPlugIn,
        IInspectable* event);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_ProcessEventAsync(This, thirdPartyPlugIn, event) \
    ((This)->lpVtbl->ProcessEventAsync(This, thirdPartyPlugIn, event))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnChannelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCredential[] = L"Windows.Networking.Vpn.IVpnCredential";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PasskeyCredential)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* get_CertificateCredential)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** value);
    HRESULT (STDMETHODCALLTYPE* get_AdditionalPin)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OldPasswordCredential)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredentialVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_get_PasskeyCredential(This, value) \
    ((This)->lpVtbl->get_PasskeyCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_get_CertificateCredential(This, value) \
    ((This)->lpVtbl->get_CertificateCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_get_AdditionalPin(This, value) \
    ((This)->lpVtbl->get_AdditionalPin(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_get_OldPasswordCredential(This, value) \
    ((This)->lpVtbl->get_OldPasswordCredential(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomCheckBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomCheckBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomCheckBox[] = L"Windows.Networking.Vpn.IVpnCustomCheckBox";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_InitialCheckState)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_InitialCheckState)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Checked)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBoxVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_put_InitialCheckState(This, value) \
    ((This)->lpVtbl->put_InitialCheckState(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_get_InitialCheckState(This, value) \
    ((This)->lpVtbl->get_InitialCheckState(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_get_Checked(This, value) \
    ((This)->lpVtbl->get_Checked(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomCheckBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomComboBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomComboBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomComboBox[] = L"Windows.Networking.Vpn.IVpnCustomComboBox";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_OptionsText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        __FIVectorView_1_HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OptionsText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Selected)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBoxVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_put_OptionsText(This, value) \
    ((This)->lpVtbl->put_OptionsText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_get_OptionsText(This, value) \
    ((This)->lpVtbl->get_OptionsText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_get_Selected(This, value) \
    ((This)->lpVtbl->get_Selected(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomComboBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomEditBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomEditBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomEditBox[] = L"Windows.Networking.Vpn.IVpnCustomEditBox";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DefaultText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NoEcho)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_NoEcho)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBoxVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_put_DefaultText(This, value) \
    ((This)->lpVtbl->put_DefaultText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_get_DefaultText(This, value) \
    ((This)->lpVtbl->get_DefaultText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_put_NoEcho(This, value) \
    ((This)->lpVtbl->put_NoEcho(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_get_NoEcho(This, value) \
    ((This)->lpVtbl->get_NoEcho(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomEditBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomErrorBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomErrorBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomErrorBox[] = L"Windows.Networking.Vpn.IVpnCustomErrorBox";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBoxVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomErrorBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPrompt[] = L"Windows.Networking.Vpn.IVpnCustomPrompt";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Label)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Compulsory)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Compulsory)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Bordered)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Bordered)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_put_Label(This, value) \
    ((This)->lpVtbl->put_Label(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_get_Label(This, value) \
    ((This)->lpVtbl->get_Label(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_put_Compulsory(This, value) \
    ((This)->lpVtbl->put_Compulsory(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_get_Compulsory(This, value) \
    ((This)->lpVtbl->get_Compulsory(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_put_Bordered(This, value) \
    ((This)->lpVtbl->put_Bordered(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_get_Bordered(This, value) \
    ((This)->lpVtbl->get_Bordered(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPrompt_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptBooleanInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptBooleanInput
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptBooleanInput[] = L"Windows.Networking.Vpn.IVpnCustomPromptBooleanInput";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_InitialValue)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_InitialValue)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInputVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_put_InitialValue(This, value) \
    ((This)->lpVtbl->put_InitialValue(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_get_InitialValue(This, value) \
    ((This)->lpVtbl->get_InitialValue(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptBooleanInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptElement[] = L"Windows.Networking.Vpn.IVpnCustomPromptElement";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Compulsory)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Compulsory)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Emphasized)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Emphasized)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElementVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_put_Compulsory(This, value) \
    ((This)->lpVtbl->put_Compulsory(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_get_Compulsory(This, value) \
    ((This)->lpVtbl->get_Compulsory(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_put_Emphasized(This, value) \
    ((This)->lpVtbl->put_Emphasized(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_get_Emphasized(This, value) \
    ((This)->lpVtbl->get_Emphasized(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptOptionSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptOptionSelector
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptOptionSelector[] = L"Windows.Networking.Vpn.IVpnCustomPromptOptionSelector";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelectorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectedIndex)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelectorVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelectorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_get_SelectedIndex(This, value) \
    ((This)->lpVtbl->get_SelectedIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptOptionSelector_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptText
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptText[] = L"Windows.Networking.Vpn.IVpnCustomPromptText";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomPromptTextInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomPromptTextInput
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomPromptTextInput[] = L"Windows.Networking.Vpn.IVpnCustomPromptTextInput";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PlaceholderText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_PlaceholderText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_IsTextHidden)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsTextHidden)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInputVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_put_PlaceholderText(This, value) \
    ((This)->lpVtbl->put_PlaceholderText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_get_PlaceholderText(This, value) \
    ((This)->lpVtbl->get_PlaceholderText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_put_IsTextHidden(This, value) \
    ((This)->lpVtbl->put_IsTextHidden(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_get_IsTextHidden(This, value) \
    ((This)->lpVtbl->get_IsTextHidden(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomPromptTextInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnCustomTextBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnCustomTextBox
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnCustomPrompt
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnCustomTextBox[] = L"Windows.Networking.Vpn.IVpnCustomTextBox";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBoxVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DisplayText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayText)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBoxVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBoxVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_put_DisplayText(This, value) \
    ((This)->lpVtbl->put_DisplayText(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_get_DisplayText(This, value) \
    ((This)->lpVtbl->get_DisplayText(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnCustomTextBox_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameAssignment[] = L"Windows.Networking.Vpn.IVpnDomainNameAssignment";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DomainNameList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** value);
    HRESULT (STDMETHODCALLTYPE* put_ProxyAutoConfigurationUri)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ProxyAutoConfigurationUri)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignmentVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_get_DomainNameList(This, value) \
    ((This)->lpVtbl->get_DomainNameList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_put_ProxyAutoConfigurationUri(This, value) \
    ((This)->lpVtbl->put_ProxyAutoConfigurationUri(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_get_ProxyAutoConfigurationUri(This, value) \
    ((This)->lpVtbl->get_ProxyAutoConfigurationUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfo[] = L"Windows.Networking.Vpn.IVpnDomainNameInfo";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DomainName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_DomainName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_DomainNameType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType value);
    HRESULT (STDMETHODCALLTYPE* get_DomainNameType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType* value);
    HRESULT (STDMETHODCALLTYPE* get_DnsServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName** value);
    HRESULT (STDMETHODCALLTYPE* get_WebProxyServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_put_DomainName(This, value) \
    ((This)->lpVtbl->put_DomainName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_get_DomainName(This, value) \
    ((This)->lpVtbl->get_DomainName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_put_DomainNameType(This, value) \
    ((This)->lpVtbl->put_DomainNameType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_get_DomainNameType(This, value) \
    ((This)->lpVtbl->get_DomainNameType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_get_DnsServers(This, value) \
    ((This)->lpVtbl->get_DnsServers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_get_WebProxyServers(This, value) \
    ((This)->lpVtbl->get_WebProxyServers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnDomainNameInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfo2[] = L"Windows.Networking.Vpn.IVpnDomainNameInfo2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WebProxyUris)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2* This,
        __FIVector_1_Windows__CFoundation__CUri** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_get_WebProxyUris(This, value) \
    ((This)->lpVtbl->get_WebProxyUris(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnDomainNameInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnDomainNameInfoFactory[] = L"Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVpnDomainNameInfo)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory* This,
        HSTRING name,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnDomainNameType nameType,
        __FIIterable_1_Windows__CNetworking__CHostName* dnsServerList,
        __FIIterable_1_Windows__CNetworking__CHostName* proxyServerList,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfo** domainNameInfo);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_CreateVpnDomainNameInfo(This, name, nameType, dnsServerList, proxyServerList, domainNameInfo) \
    ((This)->lpVtbl->CreateVpnDomainNameInfo(This, name, nameType, dnsServerList, proxyServerList, domainNameInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnDomainNameInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnForegroundActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnForegroundActivatedEventArgs[] = L"Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProfileName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        HSTRING* name);
    HRESULT (STDMETHODCALLTYPE* get_SharedContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** sharedContext);
    HRESULT (STDMETHODCALLTYPE* get_ActivationOperation)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation** activationOperation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_get_ProfileName(This, name) \
    ((This)->lpVtbl->get_ProfileName(This, name))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_get_SharedContext(This, sharedContext) \
    ((This)->lpVtbl->get_SharedContext(This, sharedContext))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_get_ActivationOperation(This, activationOperation) \
    ((This)->lpVtbl->get_ActivationOperation(This, activationOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnForegroundActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnForegroundActivationOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnForegroundActivationOperation[] = L"Windows.Networking.Vpn.IVpnForegroundActivationOperation";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperationVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_Complete(This, result) \
    ((This)->lpVtbl->Complete(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnForegroundActivationOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnInterfaceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnInterfaceId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnInterfaceId[] = L"Windows.Networking.Vpn.IVpnInterfaceId";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAddressInfo)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId* This,
        UINT32* idLength,
        BYTE** id);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_GetAddressInfo(This, idLength, id) \
    ((This)->lpVtbl->GetAddressInfo(This, idLength, id))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnInterfaceIdFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnInterfaceIdFactory[] = L"Windows.Networking.Vpn.IVpnInterfaceIdFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVpnInterfaceId)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory* This,
        UINT32 addressLength,
        BYTE* address,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceId** vpnInterfaceId);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_CreateVpnInterfaceId(This, addressLength, address, vpnInterfaceId) \
    ((This)->lpVtbl->CreateVpnInterfaceId(This, addressLength, address, vpnInterfaceId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnInterfaceIdFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnManagementAgent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnManagementAgent
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnManagementAgent[] = L"Windows.Networking.Vpn.IVpnManagementAgent";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddProfileFromXmlAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        HSTRING xml,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* AddProfileFromObjectAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateProfileFromXmlAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        HSTRING xml,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateProfileFromObjectAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* GetProfilesAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CVpn__CIVpnProfile** operation);
    HRESULT (STDMETHODCALLTYPE* DeleteProfileAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectProfileAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* ConnectProfileWithPasswordCredentialAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* passwordCredential,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);
    HRESULT (STDMETHODCALLTYPE* DisconnectProfileAsync)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* profile,
        __FIAsyncOperation_1_Windows__CNetworking__CVpn__CVpnManagementErrorStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgentVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_AddProfileFromXmlAsync(This, xml, operation) \
    ((This)->lpVtbl->AddProfileFromXmlAsync(This, xml, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_AddProfileFromObjectAsync(This, profile, operation) \
    ((This)->lpVtbl->AddProfileFromObjectAsync(This, profile, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_UpdateProfileFromXmlAsync(This, xml, operation) \
    ((This)->lpVtbl->UpdateProfileFromXmlAsync(This, xml, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_UpdateProfileFromObjectAsync(This, profile, operation) \
    ((This)->lpVtbl->UpdateProfileFromObjectAsync(This, profile, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_GetProfilesAsync(This, operation) \
    ((This)->lpVtbl->GetProfilesAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_DeleteProfileAsync(This, profile, operation) \
    ((This)->lpVtbl->DeleteProfileAsync(This, profile, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_ConnectProfileAsync(This, profile, operation) \
    ((This)->lpVtbl->ConnectProfileAsync(This, profile, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_ConnectProfileWithPasswordCredentialAsync(This, profile, passwordCredential, operation) \
    ((This)->lpVtbl->ConnectProfileWithPasswordCredentialAsync(This, profile, passwordCredential, operation))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_DisconnectProfileAsync(This, profile, operation) \
    ((This)->lpVtbl->DisconnectProfileAsync(This, profile, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnManagementAgent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNamespaceAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceAssignment[] = L"Windows.Networking.Vpn.IVpnNamespaceAssignment";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_NamespaceList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo* value);
    HRESULT (STDMETHODCALLTYPE* get_NamespaceList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnNamespaceInfo** value);
    HRESULT (STDMETHODCALLTYPE* put_ProxyAutoConfigUri)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ProxyAutoConfigUri)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignmentVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_put_NamespaceList(This, value) \
    ((This)->lpVtbl->put_NamespaceList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_get_NamespaceList(This, value) \
    ((This)->lpVtbl->get_NamespaceList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_put_ProxyAutoConfigUri(This, value) \
    ((This)->lpVtbl->put_ProxyAutoConfigUri(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_get_ProxyAutoConfigUri(This, value) \
    ((This)->lpVtbl->get_ProxyAutoConfigUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNamespaceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceInfo[] = L"Windows.Networking.Vpn.IVpnNamespaceInfo";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Namespace)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Namespace)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DnsServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_DnsServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_WebProxyServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_WebProxyServers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo* This,
        __FIVector_1_Windows__CNetworking__CHostName** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_put_Namespace(This, value) \
    ((This)->lpVtbl->put_Namespace(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_get_Namespace(This, value) \
    ((This)->lpVtbl->get_Namespace(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_put_DnsServers(This, value) \
    ((This)->lpVtbl->put_DnsServers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_get_DnsServers(This, value) \
    ((This)->lpVtbl->get_DnsServers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_put_WebProxyServers(This, value) \
    ((This)->lpVtbl->put_WebProxyServers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_get_WebProxyServers(This, value) \
    ((This)->lpVtbl->get_WebProxyServers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNamespaceInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNamespaceInfoFactory[] = L"Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVpnNamespaceInfo)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory* This,
        HSTRING name,
        __FIVector_1_Windows__CNetworking__CHostName* dnsServerList,
        __FIVector_1_Windows__CNetworking__CHostName* proxyServerList,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfo** namespaceInfo);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_CreateVpnNamespaceInfo(This, name, dnsServerList, proxyServerList, namespaceInfo) \
    ((This)->lpVtbl->CreateVpnNamespaceInfo(This, name, dnsServerList, proxyServerList, namespaceInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNamespaceInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNativeProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNativeProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNativeProfile[] = L"Windows.Networking.Vpn.IVpnNativeProfile";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Servers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RoutingPolicyType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType* value);
    HRESULT (STDMETHODCALLTYPE* put_RoutingPolicyType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType value);
    HRESULT (STDMETHODCALLTYPE* get_NativeProtocolType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnNativeProtocolType* value);
    HRESULT (STDMETHODCALLTYPE* put_NativeProtocolType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnNativeProtocolType value);
    HRESULT (STDMETHODCALLTYPE* get_UserAuthenticationMethod)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod* value);
    HRESULT (STDMETHODCALLTYPE* put_UserAuthenticationMethod)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod value);
    HRESULT (STDMETHODCALLTYPE* get_TunnelAuthenticationMethod)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod* value);
    HRESULT (STDMETHODCALLTYPE* put_TunnelAuthenticationMethod)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnAuthenticationMethod value);
    HRESULT (STDMETHODCALLTYPE* get_EapConfiguration)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        HSTRING* Value);
    HRESULT (STDMETHODCALLTYPE* put_EapConfiguration)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfileVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_Servers(This, value) \
    ((This)->lpVtbl->get_Servers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_RoutingPolicyType(This, value) \
    ((This)->lpVtbl->get_RoutingPolicyType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_put_RoutingPolicyType(This, value) \
    ((This)->lpVtbl->put_RoutingPolicyType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_NativeProtocolType(This, value) \
    ((This)->lpVtbl->get_NativeProtocolType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_put_NativeProtocolType(This, value) \
    ((This)->lpVtbl->put_NativeProtocolType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_UserAuthenticationMethod(This, value) \
    ((This)->lpVtbl->get_UserAuthenticationMethod(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_put_UserAuthenticationMethod(This, value) \
    ((This)->lpVtbl->put_UserAuthenticationMethod(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_TunnelAuthenticationMethod(This, value) \
    ((This)->lpVtbl->get_TunnelAuthenticationMethod(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_put_TunnelAuthenticationMethod(This, value) \
    ((This)->lpVtbl->put_TunnelAuthenticationMethod(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_get_EapConfiguration(This, Value) \
    ((This)->lpVtbl->get_EapConfiguration(This, Value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_put_EapConfiguration(This, value) \
    ((This)->lpVtbl->put_EapConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnNativeProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnNativeProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnNativeProfile2[] = L"Windows.Networking.Vpn.IVpnNativeProfile2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequireVpnClientAppUI)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RequireVpnClientAppUI)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionStatus)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementConnectionStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_get_RequireVpnClientAppUI(This, value) \
    ((This)->lpVtbl->get_RequireVpnClientAppUI(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_put_RequireVpnClientAppUI(This, value) \
    ((This)->lpVtbl->put_RequireVpnClientAppUI(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_get_ConnectionStatus(This, value) \
    ((This)->lpVtbl->get_ConnectionStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnNativeProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer[] = L"Windows.Networking.Vpn.IVpnPacketBuffer";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Buffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_TransportAffinity)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TransportAffinity)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_get_Buffer(This, value) \
    ((This)->lpVtbl->get_Buffer(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_put_TransportAffinity(This, value) \
    ((This)->lpVtbl->put_TransportAffinity(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_get_TransportAffinity(This, value) \
    ((This)->lpVtbl->get_TransportAffinity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer2[] = L"Windows.Networking.Vpn.IVpnPacketBuffer2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBuffer3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBuffer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBuffer3[] = L"Windows.Networking.Vpn.IVpnPacketBuffer3";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_TransportContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        IInspectable* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportContext)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_put_TransportContext(This, value) \
    ((This)->lpVtbl->put_TransportContext(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_get_TransportContext(This, value) \
    ((This)->lpVtbl->get_TransportContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferFactory[] = L"Windows.Networking.Vpn.IVpnPacketBufferFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVpnPacketBuffer)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* parentBuffer,
        UINT32 offset,
        UINT32 length,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** vpnPacketBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_CreateVpnPacketBuffer(This, parentBuffer, offset, length, vpnPacketBuffer) \
    ((This)->lpVtbl->CreateVpnPacketBuffer(This, parentBuffer, offset, length, vpnPacketBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferList[] = L"Windows.Networking.Vpn.IVpnPacketBufferList";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferListVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Append)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* AddAtBegin)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* RemoveAtBegin)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This);
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnPacketBufferStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferListVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferListVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_Append(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->Append(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_AddAtBegin(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->AddAtBegin(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_RemoveAtEnd(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->RemoveAtEnd(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_RemoveAtBegin(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->RemoveAtBegin(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPacketBufferList2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPacketBufferList2[] = L"Windows.Networking.Vpn.IVpnPacketBufferList2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddLeadingPacket)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* RemoveLeadingPacket)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* AddTrailingPacket)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* nextVpnPacketBuffer);
    HRESULT (STDMETHODCALLTYPE* RemoveTrailingPacket)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** nextVpnPacketBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_AddLeadingPacket(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->AddLeadingPacket(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_RemoveLeadingPacket(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->RemoveLeadingPacket(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_AddTrailingPacket(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->AddTrailingPacket(This, nextVpnPacketBuffer))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_RemoveTrailingPacket(This, nextVpnPacketBuffer) \
    ((This)->lpVtbl->RemoveTrailingPacket(This, nextVpnPacketBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPickedCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPickedCredential
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPickedCredential[] = L"Windows.Networking.Vpn.IVpnPickedCredential";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredentialVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PasskeyCredential)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);
    HRESULT (STDMETHODCALLTYPE* get_AdditionalPin)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OldPasswordCredential)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredentialVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredentialVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_get_PasskeyCredential(This, value) \
    ((This)->lpVtbl->get_PasskeyCredential(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_get_AdditionalPin(This, value) \
    ((This)->lpVtbl->get_AdditionalPin(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_get_OldPasswordCredential(This, value) \
    ((This)->lpVtbl->get_OldPasswordCredential(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPickedCredential_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugIn
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugIn[] = L"Windows.Networking.Vpn.IVpnPlugIn";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Connect)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel);
    HRESULT (STDMETHODCALLTYPE* Disconnect)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel);
    HRESULT (STDMETHODCALLTYPE* GetKeepAlivePayload)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer** keepAlivePacket);
    HRESULT (STDMETHODCALLTYPE* Encapsulate)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* packets,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* encapulatedPackets);
    HRESULT (STDMETHODCALLTYPE* Decapsulate)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBuffer* encapBuffer,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* decapsulatedPackets,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnPacketBufferList* controlPacketsToSend);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_Connect(This, channel) \
    ((This)->lpVtbl->Connect(This, channel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_Disconnect(This, channel) \
    ((This)->lpVtbl->Disconnect(This, channel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_GetKeepAlivePayload(This, channel, keepAlivePacket) \
    ((This)->lpVtbl->GetKeepAlivePayload(This, channel, keepAlivePacket))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_Encapsulate(This, channel, packets, encapulatedPackets) \
    ((This)->lpVtbl->Encapsulate(This, channel, packets, encapulatedPackets))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_Decapsulate(This, channel, encapBuffer, decapsulatedPackets, controlPacketsToSend) \
    ((This)->lpVtbl->Decapsulate(This, channel, encapBuffer, decapsulatedPackets, controlPacketsToSend))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugIn_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInProfile[] = L"Windows.Networking.Vpn.IVpnPlugInProfile";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServerUris)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        __FIVector_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_CustomConfiguration)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CustomConfiguration)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_VpnPluginPackageFamilyName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_VpnPluginPackageFamilyName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfileVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_get_ServerUris(This, value) \
    ((This)->lpVtbl->get_ServerUris(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_get_CustomConfiguration(This, value) \
    ((This)->lpVtbl->get_CustomConfiguration(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_put_CustomConfiguration(This, value) \
    ((This)->lpVtbl->put_CustomConfiguration(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_get_VpnPluginPackageFamilyName(This, value) \
    ((This)->lpVtbl->get_VpnPluginPackageFamilyName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_put_VpnPluginPackageFamilyName(This, value) \
    ((This)->lpVtbl->put_VpnPluginPackageFamilyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.Vpn.IVpnProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInProfile2[] = L"Windows.Networking.Vpn.IVpnPlugInProfile2";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequireVpnClientAppUI)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RequireVpnClientAppUI)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ConnectionStatus)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnManagementConnectionStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2Vtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_get_RequireVpnClientAppUI(This, value) \
    ((This)->lpVtbl->get_RequireVpnClientAppUI(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_put_RequireVpnClientAppUI(This, value) \
    ((This)->lpVtbl->put_RequireVpnClientAppUI(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_get_ConnectionStatus(This, value) \
    ((This)->lpVtbl->get_ConnectionStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnPlugInReconnectTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnPlugInReconnectTransport[] = L"Windows.Networking.Vpn.IVpnPlugInReconnectTransport";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReconnectTransport)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnChannel* channel,
        IInspectable* context);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransportVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_ReconnectTransport(This, channel, context) \
    ((This)->lpVtbl->ReconnectTransport(This, channel, context))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnPlugInReconnectTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnProfile[] = L"Windows.Networking.Vpn.IVpnProfile";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProfileName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ProfileName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AppTriggers)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnAppId** value);
    HRESULT (STDMETHODCALLTYPE* get_Routes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value);
    HRESULT (STDMETHODCALLTYPE* get_DomainNameInfoList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnDomainNameInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_TrafficFilters)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** value);
    HRESULT (STDMETHODCALLTYPE* get_RememberCredentials)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RememberCredentials)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AlwaysOn)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AlwaysOn)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfileVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_ProfileName(This, value) \
    ((This)->lpVtbl->get_ProfileName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_put_ProfileName(This, value) \
    ((This)->lpVtbl->put_ProfileName(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_AppTriggers(This, value) \
    ((This)->lpVtbl->get_AppTriggers(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_Routes(This, value) \
    ((This)->lpVtbl->get_Routes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_DomainNameInfoList(This, value) \
    ((This)->lpVtbl->get_DomainNameInfoList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_TrafficFilters(This, value) \
    ((This)->lpVtbl->get_TrafficFilters(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_RememberCredentials(This, value) \
    ((This)->lpVtbl->get_RememberCredentials(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_put_RememberCredentials(This, value) \
    ((This)->lpVtbl->put_RememberCredentials(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_get_AlwaysOn(This, value) \
    ((This)->lpVtbl->get_AlwaysOn(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_put_AlwaysOn(This, value) \
    ((This)->lpVtbl->put_AlwaysOn(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnRoute
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRoute[] = L"Windows.Networking.Vpn.IVpnRoute";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Address)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CIHostName* value);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        __x_ABI_CWindows_CNetworking_CIHostName** value);
    HRESULT (STDMETHODCALLTYPE* put_PrefixSize)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_PrefixSize)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute* This,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_put_Address(This, value) \
    ((This)->lpVtbl->put_Address(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_put_PrefixSize(This, value) \
    ((This)->lpVtbl->put_PrefixSize(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_get_PrefixSize(This, value) \
    ((This)->lpVtbl->get_PrefixSize(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRouteAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnRouteAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRouteAssignment[] = L"Windows.Networking.Vpn.IVpnRouteAssignment";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Ipv4InclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* put_Ipv6InclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* get_Ipv4InclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value);
    HRESULT (STDMETHODCALLTYPE* get_Ipv6InclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value);
    HRESULT (STDMETHODCALLTYPE* put_Ipv4ExclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* put_Ipv6ExclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute* value);
    HRESULT (STDMETHODCALLTYPE* get_Ipv4ExclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value);
    HRESULT (STDMETHODCALLTYPE* get_Ipv6ExclusionRoutes)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnRoute** value);
    HRESULT (STDMETHODCALLTYPE* put_ExcludeLocalSubnets)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ExcludeLocalSubnets)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignmentVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_put_Ipv4InclusionRoutes(This, value) \
    ((This)->lpVtbl->put_Ipv4InclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_put_Ipv6InclusionRoutes(This, value) \
    ((This)->lpVtbl->put_Ipv6InclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_get_Ipv4InclusionRoutes(This, value) \
    ((This)->lpVtbl->get_Ipv4InclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_get_Ipv6InclusionRoutes(This, value) \
    ((This)->lpVtbl->get_Ipv6InclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_put_Ipv4ExclusionRoutes(This, value) \
    ((This)->lpVtbl->put_Ipv4ExclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_put_Ipv6ExclusionRoutes(This, value) \
    ((This)->lpVtbl->put_Ipv6ExclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_get_Ipv4ExclusionRoutes(This, value) \
    ((This)->lpVtbl->get_Ipv4ExclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_get_Ipv6ExclusionRoutes(This, value) \
    ((This)->lpVtbl->get_Ipv6ExclusionRoutes(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_put_ExcludeLocalSubnets(This, value) \
    ((This)->lpVtbl->put_ExcludeLocalSubnets(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_get_ExcludeLocalSubnets(This, value) \
    ((This)->lpVtbl->get_ExcludeLocalSubnets(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnRouteFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnRouteFactory[] = L"Windows.Networking.Vpn.IVpnRouteFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVpnRoute)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory* This,
        __x_ABI_CWindows_CNetworking_CIHostName* address,
        BYTE prefixSize,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnRoute** route);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_CreateVpnRoute(This, address, prefixSize, route) \
    ((This)->lpVtbl->CreateVpnRoute(This, address, prefixSize, route))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnRouteFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnSystemHealth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnSystemHealth
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnSystemHealth[] = L"Windows.Networking.Vpn.IVpnSystemHealth";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealthVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StatementOfHealth)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealthVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealthVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_get_StatementOfHealth(This, value) \
    ((This)->lpVtbl->get_StatementOfHealth(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnSystemHealth_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilter[] = L"Windows.Networking.Vpn.IVpnTrafficFilter";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId** value);
    HRESULT (STDMETHODCALLTYPE* put_AppId)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* value);
    HRESULT (STDMETHODCALLTYPE* get_AppClaims)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_Protocol)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnIPProtocol* value);
    HRESULT (STDMETHODCALLTYPE* put_Protocol)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnIPProtocol value);
    HRESULT (STDMETHODCALLTYPE* get_LocalPortRanges)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RemotePortRanges)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalAddressRanges)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteAddressRanges)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RoutingPolicyType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType* value);
    HRESULT (STDMETHODCALLTYPE* put_RoutingPolicyType)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter* This,
        enum __x_ABI_CWindows_CNetworking_CVpn_CVpnRoutingPolicyType value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_put_AppId(This, value) \
    ((This)->lpVtbl->put_AppId(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_AppClaims(This, value) \
    ((This)->lpVtbl->get_AppClaims(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_Protocol(This, value) \
    ((This)->lpVtbl->get_Protocol(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_put_Protocol(This, value) \
    ((This)->lpVtbl->put_Protocol(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_LocalPortRanges(This, value) \
    ((This)->lpVtbl->get_LocalPortRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_RemotePortRanges(This, value) \
    ((This)->lpVtbl->get_RemotePortRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_LocalAddressRanges(This, value) \
    ((This)->lpVtbl->get_LocalAddressRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_RemoteAddressRanges(This, value) \
    ((This)->lpVtbl->get_RemoteAddressRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_get_RoutingPolicyType(This, value) \
    ((This)->lpVtbl->get_RoutingPolicyType(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_put_RoutingPolicyType(This, value) \
    ((This)->lpVtbl->put_RoutingPolicyType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilterAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilterAssignment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilterAssignment[] = L"Windows.Networking.Vpn.IVpnTrafficFilterAssignment";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TrafficFilterList)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        __FIVector_1_Windows__CNetworking__CVpn__CVpnTrafficFilter** value);
    HRESULT (STDMETHODCALLTYPE* get_AllowOutbound)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowOutbound)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowInbound)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowInbound)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignmentVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_get_TrafficFilterList(This, value) \
    ((This)->lpVtbl->get_TrafficFilterList(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_get_AllowOutbound(This, value) \
    ((This)->lpVtbl->get_AllowOutbound(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_put_AllowOutbound(This, value) \
    ((This)->lpVtbl->put_AllowOutbound(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_get_AllowInbound(This, value) \
    ((This)->lpVtbl->get_AllowInbound(This, value))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_put_AllowInbound(This, value) \
    ((This)->lpVtbl->put_AllowInbound(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterAssignment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.Vpn.IVpnTrafficFilterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.Vpn.VpnTrafficFilter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_Vpn_IVpnTrafficFilterFactory[] = L"Windows.Networking.Vpn.IVpnTrafficFilterFactory";
typedef struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory* This,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnAppId* appId,
        __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilter** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_Create(This, appId, result) \
    ((This)->lpVtbl->Create(This, appId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CVpn_CIVpnTrafficFilterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnAppId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnAppIdFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnAppId ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnAppId_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnAppId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnAppId[] = L"Windows.Networking.Vpn.VpnAppId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.Vpn.IVpnChannelStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannel ** Default Interface **
 *    Windows.Networking.Vpn.IVpnChannel2
 *    Windows.Networking.Vpn.IVpnChannel4
 *    Windows.Networking.Vpn.IVpnChannel5
 *    Windows.Networking.Vpn.IVpnChannel6
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannel_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannel[] = L"Windows.Networking.Vpn.VpnChannel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelActivityEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelActivityEventArgs ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelActivityEventArgs[] = L"Windows.Networking.Vpn.VpnChannelActivityEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelActivityStateChangedArgs[] = L"Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnChannelConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnChannelConfiguration ** Default Interface **
 *    Windows.Networking.Vpn.IVpnChannelConfiguration2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnChannelConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnChannelConfiguration[] = L"Windows.Networking.Vpn.VpnChannelConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCredential ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCredential_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCredential[] = L"Windows.Networking.Vpn.VpnCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomCheckBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomCheckBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomCheckBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomCheckBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomCheckBox[] = L"Windows.Networking.Vpn.VpnCustomCheckBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomComboBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomComboBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomComboBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomComboBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomComboBox[] = L"Windows.Networking.Vpn.VpnCustomComboBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomEditBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomEditBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomEditBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomEditBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomEditBox[] = L"Windows.Networking.Vpn.VpnCustomEditBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomErrorBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomErrorBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomErrorBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomErrorBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomErrorBox[] = L"Windows.Networking.Vpn.VpnCustomErrorBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptBooleanInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptBooleanInput ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptBooleanInput_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptBooleanInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptBooleanInput[] = L"Windows.Networking.Vpn.VpnCustomPromptBooleanInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptOptionSelector
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptOptionSelector ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptOptionSelector_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptOptionSelector_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptOptionSelector[] = L"Windows.Networking.Vpn.VpnCustomPromptOptionSelector";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptText ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptText_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptText[] = L"Windows.Networking.Vpn.VpnCustomPromptText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomPromptTextInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomPromptTextInput ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPromptElement
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptTextInput_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomPromptTextInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomPromptTextInput[] = L"Windows.Networking.Vpn.VpnCustomPromptTextInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnCustomTextBox
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnCustomTextBox ** Default Interface **
 *    Windows.Networking.Vpn.IVpnCustomPrompt
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomTextBox_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnCustomTextBox_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnCustomTextBox[] = L"Windows.Networking.Vpn.VpnCustomTextBox";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnDomainNameAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnDomainNameAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnDomainNameAssignment[] = L"Windows.Networking.Vpn.VpnDomainNameAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnDomainNameInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnDomainNameInfoFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnDomainNameInfo ** Default Interface **
 *    Windows.Networking.Vpn.IVpnDomainNameInfo2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameInfo_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnDomainNameInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnDomainNameInfo[] = L"Windows.Networking.Vpn.VpnDomainNameInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnForegroundActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnForegroundActivatedEventArgs[] = L"Windows.Networking.Vpn.VpnForegroundActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Networking.Vpn.VpnForegroundActivationOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnForegroundActivationOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivationOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnForegroundActivationOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnForegroundActivationOperation[] = L"Windows.Networking.Vpn.VpnForegroundActivationOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Networking.Vpn.VpnInterfaceId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnInterfaceIdFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnInterfaceId ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnInterfaceId_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnInterfaceId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnInterfaceId[] = L"Windows.Networking.Vpn.VpnInterfaceId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnManagementAgent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnManagementAgent ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnManagementAgent_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnManagementAgent_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnManagementAgent[] = L"Windows.Networking.Vpn.VpnManagementAgent";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNamespaceAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNamespaceAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNamespaceAssignment[] = L"Windows.Networking.Vpn.VpnNamespaceAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNamespaceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnNamespaceInfoFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNamespaceInfo ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceInfo_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNamespaceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNamespaceInfo[] = L"Windows.Networking.Vpn.VpnNamespaceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnNativeProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnNativeProfile ** Default Interface **
 *    Windows.Networking.Vpn.IVpnProfile
 *    Windows.Networking.Vpn.IVpnNativeProfile2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnNativeProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnNativeProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnNativeProfile[] = L"Windows.Networking.Vpn.VpnNativeProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPacketBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnPacketBufferFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPacketBuffer ** Default Interface **
 *    Windows.Networking.Vpn.IVpnPacketBuffer2
 *    Windows.Networking.Vpn.IVpnPacketBuffer3
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBuffer_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPacketBuffer[] = L"Windows.Networking.Vpn.VpnPacketBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPacketBufferList
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPacketBufferList ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Networking.Vpn.VpnPacketBuffer>
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBufferList_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPacketBufferList_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPacketBufferList[] = L"Windows.Networking.Vpn.VpnPacketBufferList";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPickedCredential
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPickedCredential ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPickedCredential_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPickedCredential_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPickedCredential[] = L"Windows.Networking.Vpn.VpnPickedCredential";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnPlugInProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnPlugInProfile ** Default Interface **
 *    Windows.Networking.Vpn.IVpnProfile
 *    Windows.Networking.Vpn.IVpnPlugInProfile2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnPlugInProfile_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnPlugInProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnPlugInProfile[] = L"Windows.Networking.Vpn.VpnPlugInProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnRoute
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnRouteFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnRoute ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnRoute_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnRoute_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnRoute[] = L"Windows.Networking.Vpn.VpnRoute";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnRouteAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnRouteAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnRouteAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnRouteAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnRouteAssignment[] = L"Windows.Networking.Vpn.VpnRouteAssignment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnSystemHealth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnSystemHealth ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnSystemHealth_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnSystemHealth_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnSystemHealth[] = L"Windows.Networking.Vpn.VpnSystemHealth";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnTrafficFilter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.Vpn.IVpnTrafficFilterFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnTrafficFilter ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilter_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnTrafficFilter[] = L"Windows.Networking.Vpn.VpnTrafficFilter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.Vpn.VpnTrafficFilterAssignment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.Vpn.IVpnTrafficFilterAssignment ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilterAssignment_DEFINED
#define RUNTIMECLASS_Windows_Networking_Vpn_VpnTrafficFilterAssignment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_Vpn_VpnTrafficFilterAssignment[] = L"Windows.Networking.Vpn.VpnTrafficFilterAssignment";
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
#endif // __windows2Enetworking2Evpn_p_h__

#endif // __windows2Enetworking2Evpn_h__
