
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
#ifndef __windows2Egraphics2Eprinting2Eprintsupport_h__
#define __windows2Egraphics2Eprinting2Eprintsupport_h__
#ifndef __windows2Egraphics2Eprinting2Eprintsupport_p_h__
#define __windows2Egraphics2Eprinting2Eprintsupport_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

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

#if !defined(WINDOWS_DEVICES_PRINTERS_PRINTERSCONTRACT_VERSION)
#define WINDOWS_DEVICES_PRINTERS_PRINTERSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_PRINTERS_PRINTERSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_SHELL_SECURITYAPPMANAGERCONTRACT_VERSION)

#if !defined(WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION)
#define WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_SHELL_WINDOWTABMANAGERCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.h"
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.Data.Xml.Dom.h"
#include "Windows.Devices.Printers.h"
#include "Windows.Graphics.Printing.PrintTicket.h"
#include "Windows.UI.h"
#include "Windows.UI.Shell.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportAppInfo;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportAppInfo

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportAppInfoStatics;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportAppInfoStatics

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportCommunicationErrorDetectedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportCommunicationErrorDetectedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportEnterpriseManagementUIEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportEnterpriseManagementUIEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportExtensionSession;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportExtensionSession2;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportExtensionSession3;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession3

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportExtensionTriggerDetails;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionTriggerDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportIppCommunicationConfiguration;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationConfiguration

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportIppCommunicationTimeouts;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationTimeouts

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportMxdcImageQualityConfiguration;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportMxdcImageQualityConfiguration

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesChangedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesChangedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesUpdatePolicy;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesUpdatePolicy

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintTicketElement;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketElement

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintTicketValidationRequestedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketValidationRequestedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrintTicketValidationRequestedEventArgs2;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketValidationRequestedEventArgs2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportPrinterSelectedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrinterSelectedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportSessionInfo;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSessionInfo

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportSettingsActivatedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSettingsActivatedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportSettingsActivatedEventArgs2;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2 ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSettingsActivatedEventArgs2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    interface IPrintSupportSettingsUISession;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSettingsUISession

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportPrintTicketElement;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE
#define DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("33a97a91-9bf9-59a7-9da7-b639e45f0647"))
IIterator<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*> __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_t;
#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE
#define DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e4a9120-da3e-5f31-8739-405a3b2dbfb5"))
IIterable<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketElement*> __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_t;
#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000


#ifndef DEF___FIReference_1_boolean_USE
#define DEF___FIReference_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c00fd60-2950-5939-a21a-2d12c5a01b8a"))
IReference<bool> : IReference_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<bool> __FIReference_1_boolean_t;
#define __FIReference_1_boolean ABI::Windows::Foundation::__FIReference_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_boolean_USE */


namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportExtensionSession;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportCommunicationErrorDetectedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cee7c5ea-81e8-54a0-93c6-9f926e1eeb3d"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportCommunicationErrorDetectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportCommunicationErrorDetectedEventArgs*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportCommunicationErrorDetectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession, Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportCommunicationErrorDetectedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportPrintDeviceCapabilitiesChangedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f9203731-8ec4-5001-a5b7-b67fee4ca00b"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintDeviceCapabilitiesChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintDeviceCapabilitiesChangedEventArgs*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession, Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintDeviceCapabilitiesChangedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportPrintTicketValidationRequestedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad13135a-1c8f-5ebd-a426-eb7434639d11"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketValidationRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketValidationRequestedEventArgs*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintTicketValidationRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession, Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrintTicketValidationRequestedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportPrinterSelectedEventArgs;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0cff2f5c-9b0d-56d5-afe8-c213f2b3793d"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrinterSelectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrinterSelectedEventArgs*, ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrinterSelectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession, Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportExtensionSession*, ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportPrinterSelectedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

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
        namespace ApplicationModel {
            class AppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo ABI::Windows::ApplicationModel::IAppInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument ABI::Windows::Data::Xml::Dom::IXmlDocument

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                class IppPrintDevice;
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Printers {
                interface IIppPrintDevice;
            } /* Printers */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice ABI::Windows::Devices::Printers::IIppPrintDevice

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            class Deferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IDeferral;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIDeferral ABI::Windows::Foundation::IDeferral

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

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
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    class WorkflowPrintTicket;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintTicket {
                    interface IWorkflowPrintTicket;
                } /* PrintTicket */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Shell {
                interface IAdaptiveCard;
            } /* Shell */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard ABI::Windows::UI::Shell::IAdaptiveCard

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum IppCommunicationErrorKind : int IppCommunicationErrorKind;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum IppPrinterCommunicationKind : int IppPrinterCommunicationKind;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum PrintSupportAppContracts : unsigned int PrintSupportAppContracts;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum SettingsLaunchKind : int SettingsLaunchKind;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum WorkflowPrintTicketValidationStatus : int WorkflowPrintTicketValidationStatus;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    typedef enum XpsImageQuality : int XpsImageQuality;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportAppInfo;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportIppCommunicationConfiguration;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportIppCommunicationTimeouts;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportMxdcImageQualityConfiguration;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportPrintDeviceCapabilitiesUpdatePolicy;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportSessionInfo;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    class PrintSupportSettingsUISession;
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.IppCommunicationErrorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum IppCommunicationErrorKind : int
                    {
                        IppCommunicationErrorKind_Other = 0,
                        IppCommunicationErrorKind_Timeout = 1,
                        IppCommunicationErrorKind_ConnectionError = 2,
                        IppCommunicationErrorKind_AccessDenied = 3,
                    };
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.IppPrinterCommunicationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum IppPrinterCommunicationKind : int
                    {
                        IppPrinterCommunicationKind_Network = 0,
                        IppPrinterCommunicationKind_Usb = 1,
                        IppPrinterCommunicationKind_PrinterConnection = 2,
                        IppPrinterCommunicationKind_UniversalPrint = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                        IppPrinterCommunicationKind_VirtualPrinter = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
                    };
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.PrintSupportAppContracts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum PrintSupportAppContracts : unsigned int
                    {
                        PrintSupportAppContracts_None = 0,
                        PrintSupportAppContracts_JobBackgroundSession = 0x1,
                        PrintSupportAppContracts_SettingsUI = 0x2,
                        PrintSupportAppContracts_Extension = 0x4,
                        PrintSupportAppContracts_JobUI = 0x8,
                        PrintSupportAppContracts_VirtualPrinterBackgroundSession = 0x10,
                        PrintSupportAppContracts_EnterpriseManagementUI = 0x20,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(PrintSupportAppContracts)
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum SettingsLaunchKind : int
                    {
                        SettingsLaunchKind_JobPrintTicket = 0,
                        SettingsLaunchKind_UserDefaultPrintTicket = 1,
                    };
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum WorkflowPrintTicketValidationStatus : int
                    {
                        WorkflowPrintTicketValidationStatus_Resolved = 0,
                        WorkflowPrintTicketValidationStatus_Conflicting = 1,
                        WorkflowPrintTicketValidationStatus_Invalid = 2,
                    };
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.XpsImageQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    enum XpsImageQuality : int
                    {
                        XpsImageQuality_JpegHighCompression = 0,
                        XpsImageQuality_JpegMediumCompression = 1,
                        XpsImageQuality_JpegLowCompression = 2,
                        XpsImageQuality_Png = 3,
                    };
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportAppInfo[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("913cd9e0-cb44-5ae4-b984-f6e80e872320")
                    IPrintSupportAppInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppInfo(
                            ABI::Windows::ApplicationModel::IAppInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedContracts(
                            ABI::Windows::Graphics::Printing::PrintSupport::PrintSupportAppContracts* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportAppInfo = __uuidof(IPrintSupportAppInfo);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportAppInfoStatics[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("3246a41d-2757-544d-aafa-b38461896270")
                    IPrintSupportAppInfoStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetPrintJobShowsUI(
                            HSTRING printerName,
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket* printTicket,
                            __FIReference_1_boolean** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromPrinterName(
                            HSTRING printerName,
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportAppInfo** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportAppInfoStatics = __uuidof(IPrintSupportAppInfoStatics);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportCommunicationErrorDetectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("9c90151e-ad1b-5081-a491-4a2d94244f2d")
                    IPrintSupportCommunicationErrorDetectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorKind(
                            ABI::Windows::Graphics::Printing::PrintSupport::IppCommunicationErrorKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CommunicationConfiguration(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationConfiguration** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportCommunicationErrorDetectedEventArgs = __uuidof(IPrintSupportCommunicationErrorDetectedEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportEnterpriseManagementUIEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("6b1c2850-4bf7-5894-89fa-e89d9ea4eb2e")
                    IPrintSupportEnterpriseManagementUIEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Printer(
                            ABI::Windows::Devices::Printers::IIppPrintDevice** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportEnterpriseManagementUIEventArgs = __uuidof(IPrintSupportEnterpriseManagementUIEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("eea45f1a-f4c6-54b3-a0b8-a559839aa4c3")
                    IPrintSupportExtensionSession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Printer(
                            ABI::Windows::Devices::Printers::IIppPrintDevice** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PrintTicketValidationRequested(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrintTicketValidationRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PrintDeviceCapabilitiesChanged(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrintDeviceCapabilitiesChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportExtensionSession = __uuidof(IPrintSupportExtensionSession);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("10fa8c11-6de8-5765-8fcf-e716e0f27ed1")
                    IPrintSupportExtensionSession2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_PrinterSelected(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PrinterSelected(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportExtensionSession2 = __uuidof(IPrintSupportExtensionSession2);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession3[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("0d1b755d-1273-5e14-81d3-b6bb582b9ed8")
                    IPrintSupportExtensionSession3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_CommunicationErrorDetected(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CommunicationErrorDetected(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportExtensionSession3 = __uuidof(IPrintSupportExtensionSession3);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionTriggerDetails[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("ae083711-9b09-55d1-a0ae-2a14c5f83d6a")
                    IPrintSupportExtensionTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Session(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportExtensionSession** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportExtensionTriggerDetails = __uuidof(IPrintSupportExtensionTriggerDetails);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportIppCommunicationConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("dbc36e0b-2d90-53b9-90d2-93faf30dafdd")
                    IPrintSupportIppCommunicationConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CommunicationKind(
                            ABI::Windows::Graphics::Printing::PrintSupport::IppPrinterCommunicationKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanModifyTimeouts(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IppAttributeTimeouts(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationTimeouts** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IppJobTimeouts(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationTimeouts** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportIppCommunicationConfiguration = __uuidof(IPrintSupportIppCommunicationConfiguration);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportIppCommunicationTimeouts[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("a3b2de71-564c-5806-a1a9-c6043ca5d373")
                    IPrintSupportIppCommunicationTimeouts : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ConnectTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ConnectTimeout(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SendTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SendTimeout(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ReceiveTimeout(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ReceiveTimeout(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportIppCommunicationTimeouts = __uuidof(IPrintSupportIppCommunicationTimeouts);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportMxdcImageQualityConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("0e0d0b86-d202-58a3-a1ed-2ef9dbc0f291")
                    IPrintSupportMxdcImageQualityConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NormalOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NormalOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DraftOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DraftOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HighOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HighOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PhotographicOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PhotographicOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TextOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TextOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutomaticOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AutomaticOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FaxOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FaxOutputQuality(
                            ABI::Windows::Graphics::Printing::PrintSupport::XpsImageQuality value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportMxdcImageQualityConfiguration = __uuidof(IPrintSupportMxdcImageQualityConfiguration);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("15969bf0-9028-5722-8a37-7d7c34b41dd6")
                    IPrintSupportPrintDeviceCapabilitiesChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentPrintDeviceCapabilities(
                            ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdatePrintDeviceCapabilities(
                            ABI::Windows::Data::Xml::Dom::IXmlDocument* updatedPdc
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs = __uuidof(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("469df9e7-fd07-5eeb-a07d-9fcc67f089ba")
                    IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetSupportedPdlPassthroughContentTypes(
                            __FIIterable_1_HSTRING* supportedPdlContentTypes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceLanguage(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentPrintDeviceResources(
                            ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdatePrintDeviceResources(
                            ABI::Windows::Data::Xml::Dom::IXmlDocument* updatedPdr
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPrintDeviceCapabilitiesUpdatePolicy(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesUpdatePolicy* updatePolicy
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 = __uuidof(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("d4e9b3fc-8094-5cb6-a343-ce7a97187b45")
                    IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CommunicationConfiguration(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportIppCommunicationConfiguration** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3 = __uuidof(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("31734ad5-9bfb-5bfb-bdef-8476258e3390")
                    IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MxdcImageQualityConfiguration(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportMxdcImageQualityConfiguration** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4 = __uuidof(IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesUpdatePolicy[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("5f5fc025-8c35-5529-8038-8cdc3634bbcd")
                    IPrintSupportPrintDeviceCapabilitiesUpdatePolicy : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesUpdatePolicy = __uuidof(IPrintSupportPrintDeviceCapabilitiesUpdatePolicy);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("3d9e1a70-7c39-551f-aa1f-f8ca35b3119e")
                    IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreatePeriodicRefresh(
                            ABI::Windows::Foundation::TimeSpan updatePeriod,
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesUpdatePolicy** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreatePrintJobRefresh(
                            UINT32 numberOfJobs,
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportPrintDeviceCapabilitiesUpdatePolicy** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics = __uuidof(IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketElement[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("4b2a4489-730d-5be7-80e6-8332941abf13")
                    IPrintSupportPrintTicketElement : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LocalName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_LocalName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NamespaceUri(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NamespaceUri(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintTicketElement = __uuidof(IPrintSupportPrintTicketElement);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketValidationRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("338e4e69-db55-55c7-8338-ef64680a8f90")
                    IPrintSupportPrintTicketValidationRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrintTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPrintTicketValidationStatus(
                            ABI::Windows::Graphics::Printing::PrintSupport::WorkflowPrintTicketValidationStatus status
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintTicketValidationRequestedEventArgs = __uuidof(IPrintSupportPrintTicketValidationRequestedEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketValidationRequestedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("39749075-d02d-5fd1-9c4b-46bb8db7316b")
                    IPrintSupportPrintTicketValidationRequestedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetPrintJobShowsUI(
                            boolean showsUI
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrintTicketValidationRequestedEventArgs2 = __uuidof(IPrintSupportPrintTicketValidationRequestedEventArgs2);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrinterSelectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("7b1cb7d9-a8a4-5c09-adb2-66165f817977")
                    IPrintSupportPrinterSelectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceAppInfo(
                            ABI::Windows::ApplicationModel::IAppInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PrintTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PrintTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAdditionalFeatures(
                            __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* features
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAdditionalParameters(
                            __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* parameters
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AllowedAdditionalFeaturesAndParametersCount(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAdaptiveCard(
                            ABI::Windows::UI::Shell::IAdaptiveCard* adaptiveCard
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportPrinterSelectedEventArgs = __uuidof(IPrintSupportPrinterSelectedEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSessionInfo[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("852149af-777d-53e9-9ee9-45d3f4b5be9c")
                    IPrintSupportSessionInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceAppInfo(
                            ABI::Windows::ApplicationModel::IAppInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Printer(
                            ABI::Windows::Devices::Printers::IIppPrintDevice** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportSessionInfo = __uuidof(IPrintSupportSessionInfo);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsActivatedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("1e1b565e-a013-55ea-9b8c-eea39d9fb6c1")
                    IPrintSupportSettingsActivatedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Session(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSettingsUISession** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportSettingsActivatedEventArgs = __uuidof(IPrintSupportSettingsActivatedEventArgs);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsActivatedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("abe45f6e-dc9d-5403-8107-c864d9276367")
                    IPrintSupportSettingsActivatedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OwnerWindowId(
                            ABI::Windows::UI::WindowId* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportSettingsActivatedEventArgs2 = __uuidof(IPrintSupportSettingsActivatedEventArgs2);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsUISession[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace PrintSupport {
                    MIDL_INTERFACE("c6da2251-83c3-55e4-a0f8-5de8b062adbf")
                    IPrintSupportSettingsUISession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SessionPrintTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentTitle(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LaunchKind(
                            ABI::Windows::Graphics::Printing::PrintSupport::SettingsLaunchKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdatePrintTicket(
                            ABI::Windows::Graphics::Printing::PrintTicket::IWorkflowPrintTicket* printTicket
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SessionInfo(
                            ABI::Windows::Graphics::Printing::PrintSupport::IPrintSupportSessionInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintSupportSettingsUISession = __uuidof(IPrintSupportSettingsUISession);
                } /* PrintSupport */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2 __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement;

typedef struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl;

interface __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement;

typedef struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* This,
        __FIIterator_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl;

interface __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if !defined(____FIReference_1_boolean_INTERFACE_DEFINED__)
#define ____FIReference_1_boolean_INTERFACE_DEFINED__

typedef interface __FIReference_1_boolean __FIReference_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_boolean;

typedef struct __FIReference_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIReference_1_booleanVtbl;

interface __FIReference_1_boolean
{
    CONST_VTBL struct __FIReference_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_boolean_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_boolean_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgsWithUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice;

#endif // ____x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard;

#endif // ____x_ABI_CWindows_CUI_CShell_CIAdaptiveCard_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppCommunicationErrorKind __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppCommunicationErrorKind;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppPrinterCommunicationKind __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppPrinterCommunicationKind;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CPrintSupportAppContracts __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CPrintSupportAppContracts;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CSettingsLaunchKind __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CSettingsLaunchKind;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CWorkflowPrintTicketValidationStatus __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CWorkflowPrintTicketValidationStatus;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality;

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.IppCommunicationErrorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppCommunicationErrorKind
{
    IppCommunicationErrorKind_Other = 0,
    IppCommunicationErrorKind_Timeout = 1,
    IppCommunicationErrorKind_ConnectionError = 2,
    IppCommunicationErrorKind_AccessDenied = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.IppPrinterCommunicationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppPrinterCommunicationKind
{
    IppPrinterCommunicationKind_Network = 0,
    IppPrinterCommunicationKind_Usb = 1,
    IppPrinterCommunicationKind_PrinterConnection = 2,
    IppPrinterCommunicationKind_UniversalPrint = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
    IppPrinterCommunicationKind_VirtualPrinter = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.PrintSupportAppContracts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CPrintSupportAppContracts
{
    PrintSupportAppContracts_None = 0,
    PrintSupportAppContracts_JobBackgroundSession = 0x1,
    PrintSupportAppContracts_SettingsUI = 0x2,
    PrintSupportAppContracts_Extension = 0x4,
    PrintSupportAppContracts_JobUI = 0x8,
    PrintSupportAppContracts_VirtualPrinterBackgroundSession = 0x10,
    PrintSupportAppContracts_EnterpriseManagementUI = 0x20,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CSettingsLaunchKind
{
    SettingsLaunchKind_JobPrintTicket = 0,
    SettingsLaunchKind_UserDefaultPrintTicket = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CWorkflowPrintTicketValidationStatus
{
    WorkflowPrintTicketValidationStatus_Resolved = 0,
    WorkflowPrintTicketValidationStatus_Conflicting = 1,
    WorkflowPrintTicketValidationStatus_Invalid = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.Graphics.Printing.PrintSupport.XpsImageQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality
{
    XpsImageQuality_JpegHighCompression = 0,
    XpsImageQuality_JpegMediumCompression = 1,
    XpsImageQuality_JpegLowCompression = 2,
    XpsImageQuality_Png = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportAppInfo[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppInfo)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedContracts)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CPrintSupportAppContracts* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_get_AppInfo(This, value) \
    ((This)->lpVtbl->get_AppInfo(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_get_SupportedContracts(This, value) \
    ((This)->lpVtbl->get_SupportedContracts(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportAppInfoStatics[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPrintJobShowsUI)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        HSTRING printerName,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* printTicket,
        __FIReference_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* FromPrinterName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics* This,
        HSTRING printerName,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_GetPrintJobShowsUI(This, printerName, printTicket, result) \
    ((This)->lpVtbl->GetPrintJobShowsUI(This, printerName, printTicket, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_FromPrinterName(This, printerName, result) \
    ((This)->lpVtbl->FromPrinterName(This, printerName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportAppInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportCommunicationErrorDetectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorKind)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppCommunicationErrorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_CommunicationConfiguration)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_get_ErrorKind(This, value) \
    ((This)->lpVtbl->get_ErrorKind(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_get_CommunicationConfiguration(This, value) \
    ((This)->lpVtbl->get_CommunicationConfiguration(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportCommunicationErrorDetectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportEnterpriseManagementUIEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Printer)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs* This,
        __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_get_Printer(This, value) \
    ((This)->lpVtbl->get_Printer(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportEnterpriseManagementUIEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Printer)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice** value);
    HRESULT (STDMETHODCALLTYPE* add_PrintTicketValidationRequested)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketValidationRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrintTicketValidationRequested)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PrintDeviceCapabilitiesChanged)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintDeviceCapabilitiesChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrintDeviceCapabilitiesChanged)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession* This);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSessionVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_get_Printer(This, value) \
    ((This)->lpVtbl->get_Printer(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_add_PrintTicketValidationRequested(This, handler, token) \
    ((This)->lpVtbl->add_PrintTicketValidationRequested(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_remove_PrintTicketValidationRequested(This, token) \
    ((This)->lpVtbl->remove_PrintTicketValidationRequested(This, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_add_PrintDeviceCapabilitiesChanged(This, handler, token) \
    ((This)->lpVtbl->add_PrintDeviceCapabilitiesChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_remove_PrintDeviceCapabilitiesChanged(This, token) \
    ((This)->lpVtbl->remove_PrintDeviceCapabilitiesChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PrinterSelected)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrinterSelectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PrinterSelected)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_add_PrinterSelected(This, handler, token) \
    ((This)->lpVtbl->add_PrinterSelected(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_remove_PrinterSelected(This, token) \
    ((This)->lpVtbl->remove_PrinterSelected(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionSession3[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CommunicationErrorDetected)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportExtensionSession_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportCommunicationErrorDetectedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CommunicationErrorDetected)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_add_CommunicationErrorDetected(This, handler, token) \
    ((This)->lpVtbl->add_CommunicationErrorDetected(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_remove_CommunicationErrorDetected(This, token) \
    ((This)->lpVtbl->remove_CommunicationErrorDetected(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportExtensionTriggerDetails[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionSession** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportExtensionTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportIppCommunicationConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CommunicationKind)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIppPrinterCommunicationKind* value);
    HRESULT (STDMETHODCALLTYPE* get_CanModifyTimeouts)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IppAttributeTimeouts)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts** value);
    HRESULT (STDMETHODCALLTYPE* get_IppJobTimeouts)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfigurationVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_get_CommunicationKind(This, value) \
    ((This)->lpVtbl->get_CommunicationKind(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_get_CanModifyTimeouts(This, value) \
    ((This)->lpVtbl->get_CanModifyTimeouts(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_get_IppAttributeTimeouts(This, value) \
    ((This)->lpVtbl->get_IppAttributeTimeouts(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_get_IppJobTimeouts(This, value) \
    ((This)->lpVtbl->get_IppJobTimeouts(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportIppCommunicationTimeouts[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeoutsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConnectTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_ConnectTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_SendTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_SendTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ReceiveTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_ReceiveTimeout)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeoutsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeoutsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_get_ConnectTimeout(This, value) \
    ((This)->lpVtbl->get_ConnectTimeout(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_put_ConnectTimeout(This, value) \
    ((This)->lpVtbl->put_ConnectTimeout(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_get_SendTimeout(This, value) \
    ((This)->lpVtbl->get_SendTimeout(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_put_SendTimeout(This, value) \
    ((This)->lpVtbl->put_SendTimeout(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_get_ReceiveTimeout(This, value) \
    ((This)->lpVtbl->get_ReceiveTimeout(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_put_ReceiveTimeout(This, value) \
    ((This)->lpVtbl->put_ReceiveTimeout(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationTimeouts_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportMxdcImageQualityConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NormalOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_NormalOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_DraftOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_DraftOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_HighOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_HighOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_PhotographicOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_PhotographicOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_TextOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_TextOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_AutomaticOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_AutomaticOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);
    HRESULT (STDMETHODCALLTYPE* get_FaxOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality* value);
    HRESULT (STDMETHODCALLTYPE* put_FaxOutputQuality)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CXpsImageQuality value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfigurationVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_NormalOutputQuality(This, value) \
    ((This)->lpVtbl->get_NormalOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_NormalOutputQuality(This, value) \
    ((This)->lpVtbl->put_NormalOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_DraftOutputQuality(This, value) \
    ((This)->lpVtbl->get_DraftOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_DraftOutputQuality(This, value) \
    ((This)->lpVtbl->put_DraftOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_HighOutputQuality(This, value) \
    ((This)->lpVtbl->get_HighOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_HighOutputQuality(This, value) \
    ((This)->lpVtbl->put_HighOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_PhotographicOutputQuality(This, value) \
    ((This)->lpVtbl->get_PhotographicOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_PhotographicOutputQuality(This, value) \
    ((This)->lpVtbl->put_PhotographicOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_TextOutputQuality(This, value) \
    ((This)->lpVtbl->get_TextOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_TextOutputQuality(This, value) \
    ((This)->lpVtbl->put_TextOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_AutomaticOutputQuality(This, value) \
    ((This)->lpVtbl->get_AutomaticOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_AutomaticOutputQuality(This, value) \
    ((This)->lpVtbl->put_AutomaticOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_get_FaxOutputQuality(This, value) \
    ((This)->lpVtbl->get_FaxOutputQuality(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_put_FaxOutputQuality(This, value) \
    ((This)->lpVtbl->put_FaxOutputQuality(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentPrintDeviceCapabilities)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);
    HRESULT (STDMETHODCALLTYPE* UpdatePrintDeviceCapabilities)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* updatedPdc);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_GetCurrentPrintDeviceCapabilities(This, result) \
    ((This)->lpVtbl->GetCurrentPrintDeviceCapabilities(This, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_UpdatePrintDeviceCapabilities(This, updatedPdc) \
    ((This)->lpVtbl->UpdatePrintDeviceCapabilities(This, updatedPdc))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetSupportedPdlPassthroughContentTypes)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        __FIIterable_1_HSTRING* supportedPdlContentTypes);
    HRESULT (STDMETHODCALLTYPE* get_ResourceLanguage)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentPrintDeviceResources)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);
    HRESULT (STDMETHODCALLTYPE* UpdatePrintDeviceResources)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* updatedPdr);
    HRESULT (STDMETHODCALLTYPE* SetPrintDeviceCapabilitiesUpdatePolicy)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* updatePolicy);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_SetSupportedPdlPassthroughContentTypes(This, supportedPdlContentTypes) \
    ((This)->lpVtbl->SetSupportedPdlPassthroughContentTypes(This, supportedPdlContentTypes))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_get_ResourceLanguage(This, value) \
    ((This)->lpVtbl->get_ResourceLanguage(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_GetCurrentPrintDeviceResources(This, result) \
    ((This)->lpVtbl->GetCurrentPrintDeviceResources(This, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_UpdatePrintDeviceResources(This, updatedPdr) \
    ((This)->lpVtbl->UpdatePrintDeviceResources(This, updatedPdr))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_SetPrintDeviceCapabilitiesUpdatePolicy(This, updatePolicy) \
    ((This)->lpVtbl->SetPrintDeviceCapabilitiesUpdatePolicy(This, updatePolicy))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CommunicationConfiguration)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportIppCommunicationConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_get_CommunicationConfiguration(This, value) \
    ((This)->lpVtbl->get_CommunicationConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MxdcImageQualityConfiguration)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportMxdcImageQualityConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_get_MxdcImageQualityConfiguration(This, value) \
    ((This)->lpVtbl->get_MxdcImageQualityConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesChangedEventArgs4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesUpdatePolicy[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePeriodicRefresh)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan updatePeriod,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy** result);
    HRESULT (STDMETHODCALLTYPE* CreatePrintJobRefresh)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics* This,
        UINT32 numberOfJobs,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicy** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_CreatePeriodicRefresh(This, updatePeriod, result) \
    ((This)->lpVtbl->CreatePeriodicRefresh(This, updatePeriod, result))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_CreatePrintJobRefresh(This, numberOfJobs, result) \
    ((This)->lpVtbl->CreatePrintJobRefresh(This, numberOfJobs, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketElement[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_LocalName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_NamespaceUri)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_NamespaceUri)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElementVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_get_LocalName(This, value) \
    ((This)->lpVtbl->get_LocalName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_put_LocalName(This, value) \
    ((This)->lpVtbl->put_LocalName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_get_NamespaceUri(This, value) \
    ((This)->lpVtbl->get_NamespaceUri(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_put_NamespaceUri(This, value) \
    ((This)->lpVtbl->put_NamespaceUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketValidationRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket** value);
    HRESULT (STDMETHODCALLTYPE* SetPrintTicketValidationStatus)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CWorkflowPrintTicketValidationStatus status);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_get_PrintTicket(This, value) \
    ((This)->lpVtbl->get_PrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_SetPrintTicketValidationStatus(This, status) \
    ((This)->lpVtbl->SetPrintTicketValidationStatus(This, status))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrintTicketValidationRequestedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetPrintJobShowsUI)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2* This,
        boolean showsUI);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_SetPrintJobShowsUI(This, showsUI) \
    ((This)->lpVtbl->SetPrintJobShowsUI(This, showsUI))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrintTicketValidationRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportPrinterSelectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceAppInfo)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_PrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket** value);
    HRESULT (STDMETHODCALLTYPE* put_PrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* value);
    HRESULT (STDMETHODCALLTYPE* SetAdditionalFeatures)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* features);
    HRESULT (STDMETHODCALLTYPE* SetAdditionalParameters)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __FIIterable_1_Windows__CGraphics__CPrinting__CPrintSupport__CPrintSupportPrintTicketElement* parameters);
    HRESULT (STDMETHODCALLTYPE* get_AllowedAdditionalFeaturesAndParametersCount)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SetAdaptiveCard)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CUI_CShell_CIAdaptiveCard* adaptiveCard);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_get_SourceAppInfo(This, value) \
    ((This)->lpVtbl->get_SourceAppInfo(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_get_PrintTicket(This, value) \
    ((This)->lpVtbl->get_PrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_put_PrintTicket(This, value) \
    ((This)->lpVtbl->put_PrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_SetAdditionalFeatures(This, features) \
    ((This)->lpVtbl->SetAdditionalFeatures(This, features))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_SetAdditionalParameters(This, parameters) \
    ((This)->lpVtbl->SetAdditionalParameters(This, parameters))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_get_AllowedAdditionalFeaturesAndParametersCount(This, value) \
    ((This)->lpVtbl->get_AllowedAdditionalFeaturesAndParametersCount(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_SetAdaptiveCard(This, adaptiveCard) \
    ((This)->lpVtbl->SetAdaptiveCard(This, adaptiveCard))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportPrinterSelectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSessionInfo[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceAppInfo)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Printer)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo* This,
        __x_ABI_CWindows_CDevices_CPrinters_CIIppPrintDevice** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfoVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_get_SourceAppInfo(This, value) \
    ((This)->lpVtbl->get_SourceAppInfo(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_get_Printer(This, value) \
    ((This)->lpVtbl->get_Printer(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsActivatedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Session)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_get_Session(This, value) \
    ((This)->lpVtbl->get_Session(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsActivatedEventArgs2[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OwnerWindowId)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_get_OwnerWindowId(This, value) \
    ((This)->lpVtbl->get_OwnerWindowId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_PrintSupport_IPrintSupportSettingsUISession[] = L"Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionPrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentTitle)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LaunchKind)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CSettingsLaunchKind* value);
    HRESULT (STDMETHODCALLTYPE* UpdatePrintTicket)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintTicket_CIWorkflowPrintTicket* printTicket);
    HRESULT (STDMETHODCALLTYPE* get_SessionInfo)(__x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSessionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISessionVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_get_SessionPrintTicket(This, value) \
    ((This)->lpVtbl->get_SessionPrintTicket(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_get_DocumentTitle(This, value) \
    ((This)->lpVtbl->get_DocumentTitle(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_get_LaunchKind(This, value) \
    ((This)->lpVtbl->get_LaunchKind(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_UpdatePrintTicket(This, printTicket) \
    ((This)->lpVtbl->UpdatePrintTicket(This, printTicket))

#define __x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_get_SessionInfo(This, value) \
    ((This)->lpVtbl->get_SessionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_CPrintSupport_CIPrintSupportSettingsUISession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfoStatics interface starting with version 19.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportAppInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportAppInfo[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportAppInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportCommunicationErrorDetectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportCommunicationErrorDetectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportCommunicationErrorDetectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportEnterpriseManagementUIEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportEnterpriseManagementUIEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportEnterpriseManagementUIEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession2
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionSession3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionSession[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportExtensionTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportExtensionTriggerDetails[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 17.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportIppCommunicationTimeouts ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportIppCommunicationTimeouts[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportIppCommunicationTimeouts";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x110000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportMxdcImageQualityConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportMxdcImageQualityConfiguration[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportMxdcImageQualityConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs3
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesChangedEventArgs4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintDeviceCapabilitiesUpdatePolicy ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintDeviceCapabilitiesUpdatePolicy[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketElement ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketElement[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrintTicketValidationRequestedEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrintTicketValidationRequestedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportPrinterSelectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportPrinterSelectedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSessionInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSessionInfo[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs ** Default Interface **
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsActivatedEventArgs2
 *    Windows.ApplicationModel.Activation.IActivatedEventArgs
 *    Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsActivatedEventArgs[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Class Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.PrintSupport.IPrintSupportSettingsUISession ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_PrintSupport_PrintSupportSettingsUISession[] = L"Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Eprinting2Eprintsupport_p_h__

#endif // __windows2Egraphics2Eprinting2Eprintsupport_h__
