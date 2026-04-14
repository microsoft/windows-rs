
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
#ifndef __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_h__
#define __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_h__
#ifndef __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_p_h__
#define __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_p_h__


#pragma once

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

#if !defined(WINDOWS_MANAGEMENT_DEPLOYMENT_SHAREDPACKAGECONTAINERCONTRACT_VERSION)
#define WINDOWS_MANAGEMENT_DEPLOYMENT_SHAREDPACKAGECONTAINERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MANAGEMENT_DEPLOYMENT_SHAREDPACKAGECONTAINERCONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Management.Deployment.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallItem;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallItem2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallItem3;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem3

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallItem4;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem4

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallItem5;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem5

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager3;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager3

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager4;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager4

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager5;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager5

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager6;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager6

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManager7;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager7

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallManagerItemEventArgs;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManagerItemEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallOptions;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallOptions2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallOptions2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallStatus;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallStatus

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallStatus2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallStatus2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppInstallStatus3;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallStatus3

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppUpdateOptions;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IAppUpdateOptions2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IGetEntitlementResult;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IGetEntitlementResult

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        interface IGetEntitlementResult2;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2 ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IGetEntitlementResult2

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppInstallItem;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("83b51cbf-35e0-59ad-ab3e-ffb3f03704f9"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a85c1ceb-0e8c-5422-b2ef-ad48ed338706"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class GetEntitlementResult;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4c24d7ee-4b92-5cea-a4f4-7a5d6e919062"))
IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IGetEntitlementResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("62559e90-1c0a-5708-9230-03a658652db3"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IGetEntitlementResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ccaca81b-6cf7-56f0-b7ff-8ac5191e79bf"))
IIterator<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("93e1cdc8-503f-55b1-915b-c0dc7888ce31"))
IIterable<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("48d7f874-a83c-55db-b2e6-940be9569869"))
IVectorView<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*> __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9267e107-2ac6-5e0d-86e9-3154f616c68b"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f92bfe4e-cf96-54cf-ab89-388ca004b5a9"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92730467-501e-5b05-8826-926f86925b03"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallItem*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppInstallManager;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppInstallManagerItemEventArgs;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abf1c837-dacd-5446-a032-3ee902880244"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManager*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManagerItemEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManager*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManagerItemEventArgs*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallManagerItemEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager, Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManager*, ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallManagerItemEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                class PackageVolume;
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Deployment {
                interface IPackageVolume;
            } /* Deployment */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume ABI::Windows::Management::Deployment::IPackageVolume

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        typedef enum AppInstallState : int AppInstallState;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        typedef enum AppInstallType : int AppInstallType;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        typedef enum AppInstallationToastNotificationMode : int AppInstallationToastNotificationMode;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        typedef enum AutoUpdateSetting : int AutoUpdateSetting;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        typedef enum GetEntitlementStatus : int GetEntitlementStatus;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppInstallOptions;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppInstallStatus;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        class AppUpdateOptions;
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        enum AppInstallState : int
                        {
                            AppInstallState_Pending = 0,
                            AppInstallState_Starting = 1,
                            AppInstallState_AcquiringLicense = 2,
                            AppInstallState_Downloading = 3,
                            AppInstallState_RestoringData = 4,
                            AppInstallState_Installing = 5,
                            AppInstallState_Completed = 6,
                            AppInstallState_Canceled = 7,
                            AppInstallState_Paused = 8,
                            AppInstallState_Error = 9,
                            AppInstallState_PausedLowBattery = 10,
                            AppInstallState_PausedWiFiRecommended = 11,
                            AppInstallState_PausedWiFiRequired = 12,
                            AppInstallState_ReadyToDownload = 13,
                        };
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        enum AppInstallType : int
                        {
                            AppInstallType_Install = 0,
                            AppInstallType_Update = 1,
                            AppInstallType_Repair = 2,
                        };
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        enum AppInstallationToastNotificationMode : int
                        {
                            AppInstallationToastNotificationMode_Default = 0,
                            AppInstallationToastNotificationMode_Toast = 1,
                            AppInstallationToastNotificationMode_ToastWithoutPopup = 2,
                            AppInstallationToastNotificationMode_NoToast = 3,
                        };
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        enum AutoUpdateSetting : int
                        {
                            AutoUpdateSetting_Disabled = 0,
                            AutoUpdateSetting_Enabled = 1,
                            AutoUpdateSetting_DisabledByPolicy = 2,
                            AutoUpdateSetting_EnabledByPolicy = 3,
                        };
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        enum GetEntitlementStatus : int
                        {
                            GetEntitlementStatus_Succeeded = 0,
                            GetEntitlementStatus_NoStoreAccount = 1,
                            GetEntitlementStatus_NetworkError = 2,
                            GetEntitlementStatus_ServerError = 3,
                        };
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("49d3dfab-168a-4cbf-a93a-9e448c82737d")
                        IAppInstallItem : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InstallType(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallType* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_IsUserInitiated(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetCurrentStatus(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallStatus** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Cancel(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Restart(void) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_Completed(
                                __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_Completed(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                                __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                                EventRegistrationToken token
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallItem = __uuidof(IAppInstallItem);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("d3972af8-40c0-4fd7-aa6c-0aa13ca6188c")
                        IAppInstallItem2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CancelWithTelemetry(
                                HSTRING correlationVector
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE PauseWithTelemetry(
                                HSTRING correlationVector
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RestartWithTelemetry(
                                HSTRING correlationVector
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallItem2 = __uuidof(IAppInstallItem2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("6f3dc998-dd47-433c-9234-560172d67a45")
                        IAppInstallItem3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Children(
                                __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ItemOperationsMightAffectOtherItems(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallItem3 = __uuidof(IAppInstallItem3);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem4[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("c2d1ce12-71ff-4fc8-b540-453d4b37e1d1")
                        IAppInstallItem4 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_LaunchAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_LaunchAfterInstall(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallItem4 = __uuidof(IAppInstallItem4);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem5[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("5510e7cc-4076-4a0b-9472-c21d9d380e55")
                        IAppInstallItem5 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PinToDesktopAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToDesktopAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PinToStartAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToStartAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PinToTaskbarAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToTaskbarAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CompletedInstallToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CompletedInstallToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InstallInProgressToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_InstallInProgressToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallItem5 = __uuidof(IAppInstallItem5);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("9353e170-8441-4b45-bd72-7c2fa925beee")
                        IAppInstallManager : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AppInstallItems(
                                __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Cancel(
                                HSTRING productId
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Pause(
                                HSTRING productId
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE Restart(
                                HSTRING productId
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ItemCompleted(
                                __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ItemCompleted(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE add_ItemStatusChanged(
                                __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* handler,
                                EventRegistrationToken* token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE remove_ItemStatusChanged(
                                EventRegistrationToken token
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AutoUpdateSetting(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AutoUpdateSetting* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AutoUpdateSetting(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AutoUpdateSetting value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AcquisitionIdentity(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AcquisitionIdentity(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsApplicableAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartAppInstallAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                boolean repair,
                                boolean forceUseOfNonRemovableStorage,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UpdateAppByPackageFamilyNameAsync(
                                HSTRING packageFamilyName,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForUpdatesAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForAllUpdatesAsync(
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE IsStoreBlockedByPolicyAsync(
                                HSTRING storeClientName,
                                HSTRING storeClientPublisher,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsAppAllowedToInstallAsync(
                                HSTRING productId,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager = __uuidof(IAppInstallManager);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("16937851-ed37-480d-8314-52e27c03f04a")
                        IAppInstallManager2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE StartAppInstallWithTelemetryAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                boolean repair,
                                boolean forceUseOfNonRemovableStorage,
                                HSTRING catalogId,
                                HSTRING bundleId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UpdateAppByPackageFamilyNameWithTelemetryAsync(
                                HSTRING packageFamilyName,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForUpdatesWithTelemetryAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING catalogId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForAllUpdatesWithTelemetryAsync(
                                HSTRING correlationVector,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsAppAllowedToInstallWithTelemetryAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING catalogId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CancelWithTelemetry(
                                HSTRING productId,
                                HSTRING correlationVector
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE PauseWithTelemetry(
                                HSTRING productId,
                                HSTRING correlationVector
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE RestartWithTelemetry(
                                HSTRING productId,
                                HSTRING correlationVector
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager2 = __uuidof(IAppInstallManager2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("95b24b17-e96a-4d0e-84e1-c8cb417a0178")
                        IAppInstallManager3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE StartProductInstallAsync(
                                HSTRING productId,
                                HSTRING catalogId,
                                HSTRING flightId,
                                HSTRING clientId,
                                boolean repair,
                                boolean forceUseOfNonRemovableStorage,
                                HSTRING correlationVector,
                                ABI::Windows::Management::Deployment::IPackageVolume* targetVolume,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartProductInstallForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING catalogId,
                                HSTRING flightId,
                                HSTRING clientId,
                                boolean repair,
                                boolean forceUseOfNonRemovableStorage,
                                HSTRING correlationVector,
                                ABI::Windows::Management::Deployment::IPackageVolume* targetVolume,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE UpdateAppByPackageFamilyNameForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING packageFamilyName,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForUpdatesForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING catalogId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForAllUpdatesForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsAppAllowedToInstallForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING catalogId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsApplicableForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING skuId,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE MoveToFrontOfDownloadQueue(
                                HSTRING productId,
                                HSTRING correlationVector
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager3 = __uuidof(IAppInstallManager3);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager4[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("260a2a16-5a9e-4ebd-b944-f2ba75c31159")
                        IAppInstallManager4 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE GetFreeUserEntitlementAsync(
                                HSTRING storeId,
                                HSTRING campaignId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetFreeUserEntitlementForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING storeId,
                                HSTRING campaignId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetFreeDeviceEntitlementAsync(
                                HSTRING storeId,
                                HSTRING campaignId,
                                HSTRING correlationVector,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager4 = __uuidof(IAppInstallManager4);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager5[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("3cd7be4c-1be9-4f7f-b675-aa1d64a529b2")
                        IAppInstallManager5 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AppInstallItemsWithGroupSupport(
                                __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager5 = __uuidof(IAppInstallManager5);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager6[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("c9e7d408-f27a-4471-b2f4-e76efcbebcca")
                        IAppInstallManager6 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE SearchForAllUpdatesWithUpdateOptionsAsync(
                                HSTRING correlationVector,
                                HSTRING clientId,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions* updateOptions,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForAllUpdatesWithUpdateOptionsForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING correlationVector,
                                HSTRING clientId,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions* updateOptions,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForUpdatesWithUpdateOptionsAsync(
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING correlationVector,
                                HSTRING clientId,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions* updateOptions,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE SearchForUpdatesWithUpdateOptionsForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING skuId,
                                HSTRING correlationVector,
                                HSTRING clientId,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppUpdateOptions* updateOptions,
                                __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartProductInstallWithOptionsAsync(
                                HSTRING productId,
                                HSTRING flightId,
                                HSTRING clientId,
                                HSTRING correlationVector,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallOptions* installOptions,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE StartProductInstallWithOptionsForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING productId,
                                HSTRING flightId,
                                HSTRING clientId,
                                HSTRING correlationVector,
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallOptions* installOptions,
                                __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsPackageIdentityAllowedToInstallAsync(
                                HSTRING correlationVector,
                                HSTRING packageIdentityName,
                                HSTRING publisherCertificateName,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE GetIsPackageIdentityAllowedToInstallForUserAsync(
                                ABI::Windows::System::IUser* user,
                                HSTRING correlationVector,
                                HSTRING packageIdentityName,
                                HSTRING publisherCertificateName,
                                __FIAsyncOperation_1_boolean** operation
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager6 = __uuidof(IAppInstallManager6);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager7[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("a5ee7b30-d5e4-49a3-9853-3db03203321d")
                        IAppInstallManager7 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CanInstallForAllUsers(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManager7 = __uuidof(IAppInstallManager7);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManagerItemEventArgs[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("bc505743-4674-4dd1-957e-c25682086a14")
                        IAppInstallManagerItemEventArgs : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Item(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::IAppInstallItem** value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallManagerItemEventArgs = __uuidof(IAppInstallManagerItemEventArgs);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("c9808300-1cb8-4eb6-8c9f-6a30c64a5b51")
                        IAppInstallOptions : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CatalogId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CatalogId(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ForceUseOfNonRemovableStorage(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ForceUseOfNonRemovableStorage(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AllowForcedAppRestart(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AllowForcedAppRestart(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_Repair(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_Repair(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_TargetVolume(
                                ABI::Windows::Management::Deployment::IPackageVolume** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_TargetVolume(
                                ABI::Windows::Management::Deployment::IPackageVolume* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_LaunchAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_LaunchAfterInstall(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallOptions = __uuidof(IAppInstallOptions);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallOptions2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("8a04c0d7-c94b-425e-95b4-bf27faeaee89")
                        IAppInstallOptions2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_PinToDesktopAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToDesktopAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PinToStartAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToStartAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PinToTaskbarAfterInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_PinToTaskbarAfterInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CompletedInstallToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CompletedInstallToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InstallInProgressToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_InstallInProgressToastNotificationMode(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallationToastNotificationMode value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_InstallForAllUsers(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_InstallForAllUsers(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_StageButDoNotInstall(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_StageButDoNotInstall(
                                boolean value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_CampaignId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CampaignId(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ExtendedCampaignId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_ExtendedCampaignId(
                                HSTRING value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallOptions2 = __uuidof(IAppInstallOptions2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("936dccfa-2450-4126-88b1-6127a644dd5c")
                        IAppInstallStatus : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_InstallState(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::AppInstallState* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_DownloadSizeInBytes(
                                UINT64* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_BytesDownloaded(
                                UINT64* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_PercentComplete(
                                DOUBLE* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                                HRESULT* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallStatus = __uuidof(IAppInstallStatus);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("96e7818a-5e92-4aa9-8edc-58fed4b87e00")
                        IAppInstallStatus2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_User(
                                ABI::Windows::System::IUser** value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_ReadyForLaunch(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallStatus2 = __uuidof(IAppInstallStatus2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("cb880c56-837b-4b4c-9ebb-6d44a0a96307")
                        IAppInstallStatus3 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_IsStaged(
                                boolean* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppInstallStatus3 = __uuidof(IAppInstallStatus3);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppUpdateOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("26f0b02f-c2f3-4aea-af8c-6308dd9db85f")
                        IAppUpdateOptions : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_CatalogId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_CatalogId(
                                HSTRING value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AllowForcedAppRestart(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AllowForcedAppRestart(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppUpdateOptions = __uuidof(IAppUpdateOptions);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppUpdateOptions2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("f4646e08-ed26-4bf9-9679-48f628e53df8")
                        IAppUpdateOptions2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_AutomaticallyDownloadAndInstallUpdateIfFound(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE put_AutomaticallyDownloadAndInstallUpdateIfFound(
                                boolean value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IAppUpdateOptions2 = __uuidof(IAppUpdateOptions2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IGetEntitlementResult[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("74fc843f-1a9e-4609-8e4d-819086d08a3d")
                        IGetEntitlementResult : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_Status(
                                ABI::Windows::ApplicationModel::Store::Preview::InstallControl::GetEntitlementStatus* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IGetEntitlementResult = __uuidof(IGetEntitlementResult);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IGetEntitlementResult2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Store {
                namespace Preview {
                    namespace InstallControl {
                        MIDL_INTERFACE("e3906641-a981-4302-8c68-ff836666bb3b")
                        IGetEntitlementResult2 : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE get_IsAlreadyOwned(
                                boolean* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_OrderId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_SkuId(
                                HSTRING* value
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE get_AvailabilityId(
                                HSTRING* value
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IGetEntitlementResult2 = __uuidof(IGetEntitlementResult2);
                    } /* InstallControl */
                } /* Preview */
            } /* Store */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2 __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2;

#endif // ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIIterator_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* sender,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume;

#endif // ____x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallState __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallState;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallType __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallType;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAutoUpdateSetting __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAutoUpdateSetting;

typedef enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CGetEntitlementStatus __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CGetEntitlementStatus;

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallState
{
    AppInstallState_Pending = 0,
    AppInstallState_Starting = 1,
    AppInstallState_AcquiringLicense = 2,
    AppInstallState_Downloading = 3,
    AppInstallState_RestoringData = 4,
    AppInstallState_Installing = 5,
    AppInstallState_Completed = 6,
    AppInstallState_Canceled = 7,
    AppInstallState_Paused = 8,
    AppInstallState_Error = 9,
    AppInstallState_PausedLowBattery = 10,
    AppInstallState_PausedWiFiRecommended = 11,
    AppInstallState_PausedWiFiRequired = 12,
    AppInstallState_ReadyToDownload = 13,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallType
{
    AppInstallType_Install = 0,
    AppInstallType_Update = 1,
    AppInstallType_Repair = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode
{
    AppInstallationToastNotificationMode_Default = 0,
    AppInstallationToastNotificationMode_Toast = 1,
    AppInstallationToastNotificationMode_ToastWithoutPopup = 2,
    AppInstallationToastNotificationMode_NoToast = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAutoUpdateSetting
{
    AutoUpdateSetting_Disabled = 0,
    AutoUpdateSetting_Enabled = 1,
    AutoUpdateSetting_DisabledByPolicy = 2,
    AutoUpdateSetting_EnabledByPolicy = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CGetEntitlementStatus
{
    GetEntitlementStatus_Succeeded = 0,
    GetEntitlementStatus_NoStoreAccount = 1,
    GetEntitlementStatus_NetworkError = 2,
    GetEntitlementStatus_ServerError = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InstallType)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallType* value);
    HRESULT (STDMETHODCALLTYPE* get_IsUserInitiated)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentStatus)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus** result);
    HRESULT (STDMETHODCALLTYPE* Cancel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* Restart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This);
    HRESULT (STDMETHODCALLTYPE* add_Completed)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Completed)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_get_InstallType(This, value) \
    ((This)->lpVtbl->get_InstallType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_get_IsUserInitiated(This, value) \
    ((This)->lpVtbl->get_IsUserInitiated(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_GetCurrentStatus(This, result) \
    ((This)->lpVtbl->GetCurrentStatus(This, result))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_Cancel(This) \
    ((This)->lpVtbl->Cancel(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_Restart(This) \
    ((This)->lpVtbl->Restart(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_add_Completed(This, handler, token) \
    ((This)->lpVtbl->add_Completed(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_remove_Completed(This, token) \
    ((This)->lpVtbl->remove_Completed(This, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CancelWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        HSTRING correlationVector);
    HRESULT (STDMETHODCALLTYPE* PauseWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        HSTRING correlationVector);
    HRESULT (STDMETHODCALLTYPE* RestartWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2* This,
        HSTRING correlationVector);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_CancelWithTelemetry(This, correlationVector) \
    ((This)->lpVtbl->CancelWithTelemetry(This, correlationVector))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_PauseWithTelemetry(This, correlationVector) \
    ((This)->lpVtbl->PauseWithTelemetry(This, correlationVector))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_RestartWithTelemetry(This, correlationVector) \
    ((This)->lpVtbl->RestartWithTelemetry(This, correlationVector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Children)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value);
    HRESULT (STDMETHODCALLTYPE* get_ItemOperationsMightAffectOtherItems)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_get_Children(This, value) \
    ((This)->lpVtbl->get_Children(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_get_ItemOperationsMightAffectOtherItems(This, value) \
    ((This)->lpVtbl->get_ItemOperationsMightAffectOtherItems(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem4[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LaunchAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_LaunchAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_get_LaunchAfterInstall(This, value) \
    ((This)->lpVtbl->get_LaunchAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_put_LaunchAfterInstall(This, value) \
    ((This)->lpVtbl->put_LaunchAfterInstall(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallItem5[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinToDesktopAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToDesktopAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PinToStartAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToStartAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PinToTaskbarAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToTaskbarAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CompletedInstallToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_CompletedInstallToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode value);
    HRESULT (STDMETHODCALLTYPE* get_InstallInProgressToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_InstallInProgressToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_get_PinToDesktopAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToDesktopAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_put_PinToDesktopAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToDesktopAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_get_PinToStartAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToStartAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_put_PinToStartAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToStartAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_get_PinToTaskbarAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToTaskbarAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_put_PinToTaskbarAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToTaskbarAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_get_CompletedInstallToastNotificationMode(This, value) \
    ((This)->lpVtbl->get_CompletedInstallToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_put_CompletedInstallToastNotificationMode(This, value) \
    ((This)->lpVtbl->put_CompletedInstallToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_get_InstallInProgressToastNotificationMode(This, value) \
    ((This)->lpVtbl->get_InstallInProgressToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_put_InstallInProgressToastNotificationMode(This, value) \
    ((This)->lpVtbl->put_InstallInProgressToastNotificationMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppInstallItems)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value);
    HRESULT (STDMETHODCALLTYPE* Cancel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId);
    HRESULT (STDMETHODCALLTYPE* Restart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId);
    HRESULT (STDMETHODCALLTYPE* add_ItemCompleted)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ItemCompleted)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ItemStatusChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManager_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallManagerItemEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ItemStatusChanged)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_AutoUpdateSetting)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAutoUpdateSetting* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoUpdateSetting)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAutoUpdateSetting value);
    HRESULT (STDMETHODCALLTYPE* get_AcquisitionIdentity)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AcquisitionIdentity)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* GetIsApplicableAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId,
        HSTRING skuId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* StartAppInstallAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId,
        HSTRING skuId,
        boolean repair,
        boolean forceUseOfNonRemovableStorage,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateAppByPackageFamilyNameAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING packageFamilyName,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForUpdatesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId,
        HSTRING skuId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForAllUpdatesAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* IsStoreBlockedByPolicyAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING storeClientName,
        HSTRING storeClientPublisher,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsAppAllowedToInstallAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager* This,
        HSTRING productId,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_get_AppInstallItems(This, value) \
    ((This)->lpVtbl->get_AppInstallItems(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_Cancel(This, productId) \
    ((This)->lpVtbl->Cancel(This, productId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_Pause(This, productId) \
    ((This)->lpVtbl->Pause(This, productId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_Restart(This, productId) \
    ((This)->lpVtbl->Restart(This, productId))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_add_ItemCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ItemCompleted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_remove_ItemCompleted(This, token) \
    ((This)->lpVtbl->remove_ItemCompleted(This, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_add_ItemStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_ItemStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_remove_ItemStatusChanged(This, token) \
    ((This)->lpVtbl->remove_ItemStatusChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_get_AutoUpdateSetting(This, value) \
    ((This)->lpVtbl->get_AutoUpdateSetting(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_put_AutoUpdateSetting(This, value) \
    ((This)->lpVtbl->put_AutoUpdateSetting(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_get_AcquisitionIdentity(This, value) \
    ((This)->lpVtbl->get_AcquisitionIdentity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_put_AcquisitionIdentity(This, value) \
    ((This)->lpVtbl->put_AcquisitionIdentity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_GetIsApplicableAsync(This, productId, skuId, operation) \
    ((This)->lpVtbl->GetIsApplicableAsync(This, productId, skuId, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_StartAppInstallAsync(This, productId, skuId, repair, forceUseOfNonRemovableStorage, operation) \
    ((This)->lpVtbl->StartAppInstallAsync(This, productId, skuId, repair, forceUseOfNonRemovableStorage, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_UpdateAppByPackageFamilyNameAsync(This, packageFamilyName, operation) \
    ((This)->lpVtbl->UpdateAppByPackageFamilyNameAsync(This, packageFamilyName, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_SearchForUpdatesAsync(This, productId, skuId, operation) \
    ((This)->lpVtbl->SearchForUpdatesAsync(This, productId, skuId, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_SearchForAllUpdatesAsync(This, operation) \
    ((This)->lpVtbl->SearchForAllUpdatesAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_IsStoreBlockedByPolicyAsync(This, storeClientName, storeClientPublisher, operation) \
    ((This)->lpVtbl->IsStoreBlockedByPolicyAsync(This, storeClientName, storeClientPublisher, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_GetIsAppAllowedToInstallAsync(This, productId, operation) \
    ((This)->lpVtbl->GetIsAppAllowedToInstallAsync(This, productId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAppInstallWithTelemetryAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING skuId,
        boolean repair,
        boolean forceUseOfNonRemovableStorage,
        HSTRING catalogId,
        HSTRING bundleId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateAppByPackageFamilyNameWithTelemetryAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING packageFamilyName,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForUpdatesWithTelemetryAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING skuId,
        HSTRING catalogId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForAllUpdatesWithTelemetryAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING correlationVector,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsAppAllowedToInstallWithTelemetryAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING skuId,
        HSTRING catalogId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* CancelWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING correlationVector);
    HRESULT (STDMETHODCALLTYPE* PauseWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING correlationVector);
    HRESULT (STDMETHODCALLTYPE* RestartWithTelemetry)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2* This,
        HSTRING productId,
        HSTRING correlationVector);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_StartAppInstallWithTelemetryAsync(This, productId, skuId, repair, forceUseOfNonRemovableStorage, catalogId, bundleId, correlationVector, operation) \
    ((This)->lpVtbl->StartAppInstallWithTelemetryAsync(This, productId, skuId, repair, forceUseOfNonRemovableStorage, catalogId, bundleId, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_UpdateAppByPackageFamilyNameWithTelemetryAsync(This, packageFamilyName, correlationVector, operation) \
    ((This)->lpVtbl->UpdateAppByPackageFamilyNameWithTelemetryAsync(This, packageFamilyName, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_SearchForUpdatesWithTelemetryAsync(This, productId, skuId, catalogId, correlationVector, operation) \
    ((This)->lpVtbl->SearchForUpdatesWithTelemetryAsync(This, productId, skuId, catalogId, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_SearchForAllUpdatesWithTelemetryAsync(This, correlationVector, operation) \
    ((This)->lpVtbl->SearchForAllUpdatesWithTelemetryAsync(This, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_GetIsAppAllowedToInstallWithTelemetryAsync(This, productId, skuId, catalogId, correlationVector, operation) \
    ((This)->lpVtbl->GetIsAppAllowedToInstallWithTelemetryAsync(This, productId, skuId, catalogId, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_CancelWithTelemetry(This, productId, correlationVector) \
    ((This)->lpVtbl->CancelWithTelemetry(This, productId, correlationVector))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_PauseWithTelemetry(This, productId, correlationVector) \
    ((This)->lpVtbl->PauseWithTelemetry(This, productId, correlationVector))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_RestartWithTelemetry(This, productId, correlationVector) \
    ((This)->lpVtbl->RestartWithTelemetry(This, productId, correlationVector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartProductInstallAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        HSTRING productId,
        HSTRING catalogId,
        HSTRING flightId,
        HSTRING clientId,
        boolean repair,
        boolean forceUseOfNonRemovableStorage,
        HSTRING correlationVector,
        __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume* targetVolume,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* StartProductInstallForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING catalogId,
        HSTRING flightId,
        HSTRING clientId,
        boolean repair,
        boolean forceUseOfNonRemovableStorage,
        HSTRING correlationVector,
        __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume* targetVolume,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateAppByPackageFamilyNameForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING packageFamilyName,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForUpdatesForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING skuId,
        HSTRING catalogId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForAllUpdatesForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING correlationVector,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsAppAllowedToInstallForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING skuId,
        HSTRING catalogId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsApplicableForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING skuId,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* MoveToFrontOfDownloadQueue)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3* This,
        HSTRING productId,
        HSTRING correlationVector);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_StartProductInstallAsync(This, productId, catalogId, flightId, clientId, repair, forceUseOfNonRemovableStorage, correlationVector, targetVolume, operation) \
    ((This)->lpVtbl->StartProductInstallAsync(This, productId, catalogId, flightId, clientId, repair, forceUseOfNonRemovableStorage, correlationVector, targetVolume, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_StartProductInstallForUserAsync(This, user, productId, catalogId, flightId, clientId, repair, forceUseOfNonRemovableStorage, correlationVector, targetVolume, operation) \
    ((This)->lpVtbl->StartProductInstallForUserAsync(This, user, productId, catalogId, flightId, clientId, repair, forceUseOfNonRemovableStorage, correlationVector, targetVolume, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_UpdateAppByPackageFamilyNameForUserAsync(This, user, packageFamilyName, correlationVector, operation) \
    ((This)->lpVtbl->UpdateAppByPackageFamilyNameForUserAsync(This, user, packageFamilyName, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_SearchForUpdatesForUserAsync(This, user, productId, skuId, catalogId, correlationVector, operation) \
    ((This)->lpVtbl->SearchForUpdatesForUserAsync(This, user, productId, skuId, catalogId, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_SearchForAllUpdatesForUserAsync(This, user, correlationVector, operation) \
    ((This)->lpVtbl->SearchForAllUpdatesForUserAsync(This, user, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_GetIsAppAllowedToInstallForUserAsync(This, user, productId, skuId, catalogId, correlationVector, operation) \
    ((This)->lpVtbl->GetIsAppAllowedToInstallForUserAsync(This, user, productId, skuId, catalogId, correlationVector, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_GetIsApplicableForUserAsync(This, user, productId, skuId, operation) \
    ((This)->lpVtbl->GetIsApplicableForUserAsync(This, user, productId, skuId, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_MoveToFrontOfDownloadQueue(This, productId, correlationVector) \
    ((This)->lpVtbl->MoveToFrontOfDownloadQueue(This, productId, correlationVector))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager4[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFreeUserEntitlementAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        HSTRING storeId,
        HSTRING campaignId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation);
    HRESULT (STDMETHODCALLTYPE* GetFreeUserEntitlementForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING storeId,
        HSTRING campaignId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation);
    HRESULT (STDMETHODCALLTYPE* GetFreeDeviceEntitlementAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4* This,
        HSTRING storeId,
        HSTRING campaignId,
        HSTRING correlationVector,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CGetEntitlementResult** ppAsyncOperation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetFreeUserEntitlementAsync(This, storeId, campaignId, correlationVector, ppAsyncOperation) \
    ((This)->lpVtbl->GetFreeUserEntitlementAsync(This, storeId, campaignId, correlationVector, ppAsyncOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetFreeUserEntitlementForUserAsync(This, user, storeId, campaignId, correlationVector, ppAsyncOperation) \
    ((This)->lpVtbl->GetFreeUserEntitlementForUserAsync(This, user, storeId, campaignId, correlationVector, ppAsyncOperation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_GetFreeDeviceEntitlementAsync(This, storeId, campaignId, correlationVector, ppAsyncOperation) \
    ((This)->lpVtbl->GetFreeDeviceEntitlementAsync(This, storeId, campaignId, correlationVector, ppAsyncOperation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager5[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppInstallItemsWithGroupSupport)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5* This,
        __FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_get_AppInstallItemsWithGroupSupport(This, value) \
    ((This)->lpVtbl->get_AppInstallItemsWithGroupSupport(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager6[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SearchForAllUpdatesWithUpdateOptionsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        HSTRING correlationVector,
        HSTRING clientId,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* updateOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForAllUpdatesWithUpdateOptionsForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING correlationVector,
        HSTRING clientId,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* updateOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForUpdatesWithUpdateOptionsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        HSTRING productId,
        HSTRING skuId,
        HSTRING correlationVector,
        HSTRING clientId,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* updateOptions,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* SearchForUpdatesWithUpdateOptionsForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING skuId,
        HSTRING correlationVector,
        HSTRING clientId,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* updateOptions,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* StartProductInstallWithOptionsAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        HSTRING productId,
        HSTRING flightId,
        HSTRING clientId,
        HSTRING correlationVector,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* installOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* StartProductInstallWithOptionsForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING productId,
        HSTRING flightId,
        HSTRING clientId,
        HSTRING correlationVector,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* installOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStore__CPreview__CInstallControl__CAppInstallItem** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsPackageIdentityAllowedToInstallAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        HSTRING correlationVector,
        HSTRING packageIdentityName,
        HSTRING publisherCertificateName,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* GetIsPackageIdentityAllowedToInstallForUserAsync)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING correlationVector,
        HSTRING packageIdentityName,
        HSTRING publisherCertificateName,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_SearchForAllUpdatesWithUpdateOptionsAsync(This, correlationVector, clientId, updateOptions, operation) \
    ((This)->lpVtbl->SearchForAllUpdatesWithUpdateOptionsAsync(This, correlationVector, clientId, updateOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_SearchForAllUpdatesWithUpdateOptionsForUserAsync(This, user, correlationVector, clientId, updateOptions, operation) \
    ((This)->lpVtbl->SearchForAllUpdatesWithUpdateOptionsForUserAsync(This, user, correlationVector, clientId, updateOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_SearchForUpdatesWithUpdateOptionsAsync(This, productId, skuId, correlationVector, clientId, updateOptions, operation) \
    ((This)->lpVtbl->SearchForUpdatesWithUpdateOptionsAsync(This, productId, skuId, correlationVector, clientId, updateOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_SearchForUpdatesWithUpdateOptionsForUserAsync(This, user, productId, skuId, correlationVector, clientId, updateOptions, operation) \
    ((This)->lpVtbl->SearchForUpdatesWithUpdateOptionsForUserAsync(This, user, productId, skuId, correlationVector, clientId, updateOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_StartProductInstallWithOptionsAsync(This, productId, flightId, clientId, correlationVector, installOptions, operation) \
    ((This)->lpVtbl->StartProductInstallWithOptionsAsync(This, productId, flightId, clientId, correlationVector, installOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_StartProductInstallWithOptionsForUserAsync(This, user, productId, flightId, clientId, correlationVector, installOptions, operation) \
    ((This)->lpVtbl->StartProductInstallWithOptionsForUserAsync(This, user, productId, flightId, clientId, correlationVector, installOptions, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_GetIsPackageIdentityAllowedToInstallAsync(This, correlationVector, packageIdentityName, publisherCertificateName, operation) \
    ((This)->lpVtbl->GetIsPackageIdentityAllowedToInstallAsync(This, correlationVector, packageIdentityName, publisherCertificateName, operation))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_GetIsPackageIdentityAllowedToInstallForUserAsync(This, user, correlationVector, packageIdentityName, publisherCertificateName, operation) \
    ((This)->lpVtbl->GetIsPackageIdentityAllowedToInstallForUserAsync(This, user, correlationVector, packageIdentityName, publisherCertificateName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManager7[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanInstallForAllUsers)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_get_CanInstallForAllUsers(This, value) \
    ((This)->lpVtbl->get_CanInstallForAllUsers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManager7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallManagerItemEventArgs[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Item)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallItem** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_get_Item(This, value) \
    ((This)->lpVtbl->get_Item(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallManagerItemEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CatalogId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CatalogId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ForceUseOfNonRemovableStorage)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ForceUseOfNonRemovableStorage)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowForcedAppRestart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowForcedAppRestart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Repair)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Repair)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TargetVolume)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume** value);
    HRESULT (STDMETHODCALLTYPE* put_TargetVolume)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        __x_ABI_CWindows_CManagement_CDeployment_CIPackageVolume* value);
    HRESULT (STDMETHODCALLTYPE* get_LaunchAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_LaunchAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_CatalogId(This, value) \
    ((This)->lpVtbl->get_CatalogId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_CatalogId(This, value) \
    ((This)->lpVtbl->put_CatalogId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_ForceUseOfNonRemovableStorage(This, value) \
    ((This)->lpVtbl->get_ForceUseOfNonRemovableStorage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_ForceUseOfNonRemovableStorage(This, value) \
    ((This)->lpVtbl->put_ForceUseOfNonRemovableStorage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_AllowForcedAppRestart(This, value) \
    ((This)->lpVtbl->get_AllowForcedAppRestart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_AllowForcedAppRestart(This, value) \
    ((This)->lpVtbl->put_AllowForcedAppRestart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_Repair(This, value) \
    ((This)->lpVtbl->get_Repair(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_Repair(This, value) \
    ((This)->lpVtbl->put_Repair(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_TargetVolume(This, value) \
    ((This)->lpVtbl->get_TargetVolume(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_TargetVolume(This, value) \
    ((This)->lpVtbl->put_TargetVolume(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_get_LaunchAfterInstall(This, value) \
    ((This)->lpVtbl->get_LaunchAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_put_LaunchAfterInstall(This, value) \
    ((This)->lpVtbl->put_LaunchAfterInstall(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallOptions2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PinToDesktopAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToDesktopAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PinToStartAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToStartAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PinToTaskbarAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PinToTaskbarAfterInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CompletedInstallToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_CompletedInstallToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode value);
    HRESULT (STDMETHODCALLTYPE* get_InstallInProgressToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_InstallInProgressToastNotificationMode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallationToastNotificationMode value);
    HRESULT (STDMETHODCALLTYPE* get_InstallForAllUsers)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_InstallForAllUsers)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StageButDoNotInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_StageButDoNotInstall)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CampaignId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CampaignId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedCampaignId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ExtendedCampaignId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_PinToDesktopAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToDesktopAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_PinToDesktopAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToDesktopAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_PinToStartAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToStartAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_PinToStartAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToStartAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_PinToTaskbarAfterInstall(This, value) \
    ((This)->lpVtbl->get_PinToTaskbarAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_PinToTaskbarAfterInstall(This, value) \
    ((This)->lpVtbl->put_PinToTaskbarAfterInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_CompletedInstallToastNotificationMode(This, value) \
    ((This)->lpVtbl->get_CompletedInstallToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_CompletedInstallToastNotificationMode(This, value) \
    ((This)->lpVtbl->put_CompletedInstallToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_InstallInProgressToastNotificationMode(This, value) \
    ((This)->lpVtbl->get_InstallInProgressToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_InstallInProgressToastNotificationMode(This, value) \
    ((This)->lpVtbl->put_InstallInProgressToastNotificationMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_InstallForAllUsers(This, value) \
    ((This)->lpVtbl->get_InstallForAllUsers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_InstallForAllUsers(This, value) \
    ((This)->lpVtbl->put_InstallForAllUsers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_StageButDoNotInstall(This, value) \
    ((This)->lpVtbl->get_StageButDoNotInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_StageButDoNotInstall(This, value) \
    ((This)->lpVtbl->put_StageButDoNotInstall(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_CampaignId(This, value) \
    ((This)->lpVtbl->get_CampaignId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_CampaignId(This, value) \
    ((This)->lpVtbl->put_CampaignId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_get_ExtendedCampaignId(This, value) \
    ((This)->lpVtbl->get_ExtendedCampaignId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_put_ExtendedCampaignId(This, value) \
    ((This)->lpVtbl->put_ExtendedCampaignId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InstallState)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CAppInstallState* value);
    HRESULT (STDMETHODCALLTYPE* get_DownloadSizeInBytes)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_BytesDownloaded)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_PercentComplete)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatusVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_get_InstallState(This, value) \
    ((This)->lpVtbl->get_InstallState(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_get_DownloadSizeInBytes(This, value) \
    ((This)->lpVtbl->get_DownloadSizeInBytes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_get_BytesDownloaded(This, value) \
    ((This)->lpVtbl->get_BytesDownloaded(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_get_PercentComplete(This, value) \
    ((This)->lpVtbl->get_PercentComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* get_ReadyForLaunch)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_get_ReadyForLaunch(This, value) \
    ((This)->lpVtbl->get_ReadyForLaunch(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppInstallStatus3[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsStaged)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_get_IsStaged(This, value) \
    ((This)->lpVtbl->get_IsStaged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppInstallStatus3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppUpdateOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CatalogId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CatalogId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AllowForcedAppRestart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowForcedAppRestart)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_get_CatalogId(This, value) \
    ((This)->lpVtbl->get_CatalogId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_put_CatalogId(This, value) \
    ((This)->lpVtbl->put_CatalogId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_get_AllowForcedAppRestart(This, value) \
    ((This)->lpVtbl->get_AllowForcedAppRestart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_put_AllowForcedAppRestart(This, value) \
    ((This)->lpVtbl->put_AllowForcedAppRestart(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IAppUpdateOptions2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutomaticallyDownloadAndInstallUpdateIfFound)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AutomaticallyDownloadAndInstallUpdateIfFound)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_get_AutomaticallyDownloadAndInstallUpdateIfFound(This, value) \
    ((This)->lpVtbl->get_AutomaticallyDownloadAndInstallUpdateIfFound(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_put_AutomaticallyDownloadAndInstallUpdateIfFound(This, value) \
    ((This)->lpVtbl->put_AutomaticallyDownloadAndInstallUpdateIfFound(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIAppUpdateOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IGetEntitlementResult[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CGetEntitlementStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Store_Preview_InstallControl_IGetEntitlementResult2[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2";
typedef struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAlreadyOwned)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_OrderId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SkuId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailabilityId)(__x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_get_IsAlreadyOwned(This, value) \
    ((This)->lpVtbl->get_IsAlreadyOwned(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_get_OrderId(This, value) \
    ((This)->lpVtbl->get_OrderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_get_SkuId(This, value) \
    ((This)->lpVtbl->get_SkuId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_get_AvailabilityId(This, value) \
    ((This)->lpVtbl->get_AvailabilityId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CStore_CPreview_CInstallControl_CIGetEntitlementResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallItem[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManager[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallManagerItemEventArgs[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppInstallStatus[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_AppUpdateOptions[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult ** Default Interface **
 *    Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Store_Preview_InstallControl_GetEntitlementResult[] = L"Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
#endif // __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_p_h__

#endif // __windows2Eapplicationmodel2Estore2Epreview2Einstallcontrol_h__
