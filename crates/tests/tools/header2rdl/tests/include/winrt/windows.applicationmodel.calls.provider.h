
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
#ifndef __windows2Eapplicationmodel2Ecalls2Eprovider_h__
#define __windows2Eapplicationmodel2Ecalls2Eprovider_h__
#ifndef __windows2Eapplicationmodel2Ecalls2Eprovider_p_h__
#define __windows2Eapplicationmodel2Ecalls2Eprovider_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Calls.h"
#include "Windows.Storage.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOrigin;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOrigin

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOrigin2;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2 ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOrigin2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOrigin3;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3 ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOrigin3

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOriginManagerStatics;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOriginManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOriginManagerStatics2;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2 ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOriginManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    interface IPhoneCallOriginManagerStatics3;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3 ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOriginManagerStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__

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
        namespace Storage {
            class StorageFile;
        } /* Storage */
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
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    class PhoneCallOrigin;
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("20613479-0ef9-4454-871c-afb66a14b6a5")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOrigin : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_Category(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE put_Category(
                            HSTRING value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_CategoryDescription(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE put_CategoryDescription(
                            HSTRING value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_Location(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE put_Location(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOrigin = __uuidof(IPhoneCallOrigin);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin2[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("04c7e980-9ac2-4768-b536-b68da4957d02")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOrigin2 : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOrigin2 = __uuidof(IPhoneCallOrigin2);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin3[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("49330fb4-d1a7-43a2-aeee-c07b6dbaf068")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOrigin3 : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayPicture(
                            ABI::Windows::Storage::IStorageFile** value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE put_DisplayPicture(
                            ABI::Windows::Storage::IStorageFile* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOrigin3 = __uuidof(IPhoneCallOrigin3);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("ccfc5a0a-9af7-6149-39d0-e076fcce1395")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOriginManagerStatics : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_IsCurrentAppActiveCallOriginApp(
                            boolean* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE ShowPhoneCallOriginSettingsUI(void) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE SetCallOrigin(
                            GUID requestId,
                            ABI::Windows::ApplicationModel::Calls::Provider::IPhoneCallOrigin* callOrigin
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOriginManagerStatics = __uuidof(IPhoneCallOriginManagerStatics);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics2[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("8bf3ee3f-40f4-4380-8c7c-aea2c9b8dd7a")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOriginManagerStatics2 : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE RequestSetAsActiveCallOriginAppAsync(
                            __FIAsyncOperation_1_boolean** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOriginManagerStatics2 = __uuidof(IPhoneCallOriginManagerStatics2);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics3[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Provider {
                    MIDL_INTERFACE("2ed69764-a6e3-50f0-b76a-d67cb39bdfde")
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    IPhoneCallOriginManagerStatics3 : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                        virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOriginManagerStatics3 = __uuidof(IPhoneCallOriginManagerStatics3);
                } /* Provider */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin ** Default Interface **
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin[] = L"Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3 interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2 interface starting with version 3.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager[] = L"Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2 __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3 __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2 __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3 __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_Category)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_CategoryDescription)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_CategoryDescription)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_Location)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_Location)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_put_Category(This, value) \
    ((This)->lpVtbl->put_Category(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_get_CategoryDescription(This, value) \
    ((This)->lpVtbl->get_CategoryDescription(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_put_CategoryDescription(This, value) \
    ((This)->lpVtbl->put_CategoryDescription(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_get_Location(This, value) \
    ((This)->lpVtbl->get_Location(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_put_Location(This, value) \
    ((This)->lpVtbl->put_Location(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin2[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOrigin3[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_DisplayPicture)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* put_DisplayPicture)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_get_DisplayPicture(This, value) \
    ((This)->lpVtbl->get_DisplayPicture(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_put_DisplayPicture(This, value) \
    ((This)->lpVtbl->put_DisplayPicture(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_IsCurrentAppActiveCallOriginApp)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        boolean* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* ShowPhoneCallOriginSettingsUI)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* SetCallOrigin)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics* This,
        GUID requestId,
        __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOrigin* callOrigin);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_get_IsCurrentAppActiveCallOriginApp(This, value) \
    ((This)->lpVtbl->get_IsCurrentAppActiveCallOriginApp(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_ShowPhoneCallOriginSettingsUI(This) \
    ((This)->lpVtbl->ShowPhoneCallOriginSettingsUI(This))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_SetCallOrigin(This, requestId, callOrigin) \
    ((This)->lpVtbl->SetCallOrigin(This, requestId, callOrigin))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics2[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* RequestSetAsActiveCallOriginAppAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2* This,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_RequestSetAsActiveCallOriginAppAsync(This, result) \
    ((This)->lpVtbl->RequestSetAsActiveCallOriginAppAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Provider_IPhoneCallOriginManagerStatics3[] = L"Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CProvider_CIPhoneCallOriginManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin ** Default Interface **
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin2
 *    Windows.ApplicationModel.Calls.Provider.IPhoneCallOrigin3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOrigin is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Provider_PhoneCallOrigin[] = L"Windows.ApplicationModel.Calls.Provider.PhoneCallOrigin";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics3 interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.Provider.IPhoneCallOriginManagerStatics2 interface starting with version 3.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
DEPRECATED("PhoneCallOriginManager is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Provider_PhoneCallOriginManager[] = L"Windows.ApplicationModel.Calls.Provider.PhoneCallOriginManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Ecalls2Eprovider_p_h__

#endif // __windows2Eapplicationmodel2Ecalls2Eprovider_h__
