
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
#ifndef __windows2Eui2Enotifications2Emanagement_h__
#define __windows2Eui2Enotifications2Emanagement_h__
#ifndef __windows2Eui2Enotifications2Emanagement_p_h__
#define __windows2Eui2Enotifications2Emanagement_p_h__


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
#include "Windows.UI.Notifications.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    interface IUserNotificationListener;
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener ABI::Windows::UI::Notifications::Management::IUserNotificationListener

#endif // ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    interface IUserNotificationListenerStatics;
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics ABI::Windows::UI::Notifications::Management::IUserNotificationListenerStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class UserNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IUserNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification ABI::Windows::UI::Notifications::IUserNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CUserNotification_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CUserNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6ee1200d-dd13-5050-88cb-5352af113fd1"))
IIterator<ABI::Windows::UI::Notifications::UserNotification*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::UserNotification*, ABI::Windows::UI::Notifications::IUserNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.UserNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::UserNotification*> __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CUserNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CUserNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CUserNotification_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CUserNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("18170480-1bd8-5cd0-bb32-67e71d5b4d7c"))
IIterable<ABI::Windows::UI::Notifications::UserNotification*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::UserNotification*, ABI::Windows::UI::Notifications::IUserNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.UserNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::UserNotification*> __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CUserNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CUserNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5a08f98c-8e45-5705-af54-f5418e598392"))
IVectorView<ABI::Windows::UI::Notifications::UserNotification*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::UserNotification*, ABI::Windows::UI::Notifications::IUserNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.UserNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::UserNotification*> __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bf7f3d3c-230f-54ea-ad74-0cf6c55cd8d1"))
IAsyncOperation<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.UserNotification>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*> __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9e42ed08-45b3-5643-b5c7-b216f5781594"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.UserNotification>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    typedef enum UserNotificationListenerAccessStatus : int UserNotificationListenerAccessStatus;
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0fbad8c7-086f-5bf9-81e2-8d79e7184803"))
IAsyncOperation<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus> __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f09e843a-13cb-559b-a9fc-015722c2cd57"))
IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    class UserNotificationListener;
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class UserNotificationChangedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IUserNotificationChangedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs ABI::Windows::UI::Notifications::IUserNotificationChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10242902-b897-5507-9922-2c0a7d34464d"))
ITypedEventHandler<ABI::Windows::UI::Notifications::Management::UserNotificationListener*, ABI::Windows::UI::Notifications::UserNotificationChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::Management::UserNotificationListener*, ABI::Windows::UI::Notifications::Management::IUserNotificationListener*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::UserNotificationChangedEventArgs*, ABI::Windows::UI::Notifications::IUserNotificationChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.Management.UserNotificationListener, Windows.UI.Notifications.UserNotificationChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::Management::UserNotificationListener*, ABI::Windows::UI::Notifications::UserNotificationChangedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum NotificationKinds : unsigned int NotificationKinds;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    enum UserNotificationListenerAccessStatus : int
                    {
                        UserNotificationListenerAccessStatus_Unspecified = 0,
                        UserNotificationListenerAccessStatus_Allowed = 1,
                        UserNotificationListenerAccessStatus_Denied = 2,
                    };
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.Management.IUserNotificationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Management.UserNotificationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Management_IUserNotificationListener[] = L"Windows.UI.Notifications.Management.IUserNotificationListener";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    MIDL_INTERFACE("62553e41-8a06-4cef-8215-6033a5be4b03")
                    IUserNotificationListener : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                            __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAccessStatus(
                            ABI::Windows::UI::Notifications::Management::UserNotificationListenerAccessStatus* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_NotificationChanged(
                            __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NotificationChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNotificationsAsync(
                            ABI::Windows::UI::Notifications::NotificationKinds kinds,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNotification(
                            UINT32 notificationId,
                            ABI::Windows::UI::Notifications::IUserNotification** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ClearNotifications(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveNotification(
                            UINT32 notificationId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserNotificationListener = __uuidof(IUserNotificationListener);
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.Management.IUserNotificationListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Management.UserNotificationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Management_IUserNotificationListenerStatics[] = L"Windows.UI.Notifications.Management.IUserNotificationListenerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                namespace Management {
                    MIDL_INTERFACE("ff6123cf-4386-4aa3-b73d-b804e5b63b23")
                    IUserNotificationListenerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Current(
                            ABI::Windows::UI::Notifications::Management::IUserNotificationListener** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserNotificationListenerStatics = __uuidof(IUserNotificationListenerStatics);
                } /* Management */
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.Management.UserNotificationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.Management.IUserNotificationListenerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.Management.IUserNotificationListener ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Management_UserNotificationListener_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Management_UserNotificationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Management_UserNotificationListener[] = L"Windows.UI.Notifications.Management.UserNotificationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotification __x_ABI_CWindows_CUI_CNotifications_CIUserNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CUserNotification __FIIterator_1_Windows__CUI__CNotifications__CUserNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CUserNotification;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CUserNotification* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CUserNotificationVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CUserNotification
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CUserNotification_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CUserNotification __FIIterable_1_Windows__CUI__CNotifications__CUserNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CUserNotification;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CUserNotification* This,
        __FIIterator_1_Windows__CUI__CNotifications__CUserNotification** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CUserNotificationVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CUserNotification
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CUserNotification_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CUserNotification** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CUI_CNotifications_CManagement_CUserNotificationListenerAccessStatus __x_ABI_CWindows_CUI_CNotifications_CManagement_CUserNotificationListenerAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CManagement_CUserNotificationListenerAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* This,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* sender,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CUI_CNotifications_CNotificationKinds __x_ABI_CWindows_CUI_CNotifications_CNotificationKinds;

/*
 *
 * Struct Windows.UI.Notifications.Management.UserNotificationListenerAccessStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CNotifications_CManagement_CUserNotificationListenerAccessStatus
{
    UserNotificationListenerAccessStatus_Unspecified = 0,
    UserNotificationListenerAccessStatus_Allowed = 1,
    UserNotificationListenerAccessStatus_Denied = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.Management.IUserNotificationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Management.UserNotificationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Management_IUserNotificationListener[] = L"Windows.UI.Notifications.Management.IUserNotificationListener";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CManagement__CUserNotificationListenerAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* GetAccessStatus)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CManagement_CUserNotificationListenerAccessStatus* result);
    HRESULT (STDMETHODCALLTYPE* add_NotificationChanged)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CManagement__CUserNotificationListener_Windows__CUI__CNotifications__CUserNotificationChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NotificationChanged)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetNotificationsAsync)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationKinds kinds,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CUserNotification** operation);
    HRESULT (STDMETHODCALLTYPE* GetNotification)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        UINT32 notificationId,
        __x_ABI_CWindows_CUI_CNotifications_CIUserNotification** result);
    HRESULT (STDMETHODCALLTYPE* ClearNotifications)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This);
    HRESULT (STDMETHODCALLTYPE* RemoveNotification)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener* This,
        UINT32 notificationId);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetAccessStatus(This, result) \
    ((This)->lpVtbl->GetAccessStatus(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_add_NotificationChanged(This, handler, token) \
    ((This)->lpVtbl->add_NotificationChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_remove_NotificationChanged(This, token) \
    ((This)->lpVtbl->remove_NotificationChanged(This, token))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetNotificationsAsync(This, kinds, operation) \
    ((This)->lpVtbl->GetNotificationsAsync(This, kinds, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_GetNotification(This, notificationId, result) \
    ((This)->lpVtbl->GetNotification(This, notificationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_ClearNotifications(This) \
    ((This)->lpVtbl->ClearNotifications(This))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_RemoveNotification(This, notificationId) \
    ((This)->lpVtbl->RemoveNotification(This, notificationId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.Management.IUserNotificationListenerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Management.UserNotificationListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_Management_IUserNotificationListenerStatics[] = L"Windows.UI.Notifications.Management.IUserNotificationListenerStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListener** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CManagement_CIUserNotificationListenerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.Management.UserNotificationListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.Management.IUserNotificationListenerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.Management.IUserNotificationListener ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Management_UserNotificationListener_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Management_UserNotificationListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Management_UserNotificationListener[] = L"Windows.UI.Notifications.Management.UserNotificationListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Enotifications2Emanagement_p_h__

#endif // __windows2Eui2Enotifications2Emanagement_h__
