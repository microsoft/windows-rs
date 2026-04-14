
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
#ifndef __windows2Ephone2Estartscreen_h__
#define __windows2Ephone2Estartscreen_h__
#ifndef __windows2Ephone2Estartscreen_p_h__
#define __windows2Ephone2Estartscreen_p_h__


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

#if !defined(WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION)
#define WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.Notifications.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                interface IDualSimTile;
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile ABI::Windows::Phone::StartScreen::IDualSimTile

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                interface IDualSimTileStatics;
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics ABI::Windows::Phone::StartScreen::IDualSimTileStatics

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                interface IToastNotificationManagerStatics3;
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3 ABI::Windows::Phone::StartScreen::IToastNotificationManagerStatics3

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__

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
        namespace UI {
            namespace Notifications {
                class BadgeUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater ABI::Windows::UI::Notifications::IBadgeUpdater

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater ABI::Windows::UI::Notifications::ITileUpdater

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotifier;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotifier;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier ABI::Windows::UI::Notifications::IToastNotifier

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                class DualSimTile;
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Phone.StartScreen.IDualSimTile
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.StartScreen.DualSimTile
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IDualSimTile[] = L"Windows.Phone.StartScreen.IDualSimTile";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                MIDL_INTERFACE("143ab213-d05f-4041-a18c-3e3fcb75b41e")
                IDualSimTile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPinnedToStart(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDualSimTile = __uuidof(IDualSimTile);
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.StartScreen.IDualSimTileStatics
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.StartScreen.DualSimTile
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IDualSimTileStatics[] = L"Windows.Phone.StartScreen.IDualSimTileStatics";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                MIDL_INTERFACE("50567c9e-c58f-4dc9-b6e8-fa6777eeeb37")
                IDualSimTileStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetTileForSim2(
                        ABI::Windows::Phone::StartScreen::IDualSimTile** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateDisplayNameForSim1Async(
                        HSTRING name,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForSim1(
                        ABI::Windows::UI::Notifications::ITileUpdater** updater
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForSim2(
                        ABI::Windows::UI::Notifications::ITileUpdater** updater
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForSim1(
                        ABI::Windows::UI::Notifications::IBadgeUpdater** updater
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForSim2(
                        ABI::Windows::UI::Notifications::IBadgeUpdater** updater
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifierForSim1(
                        ABI::Windows::UI::Notifications::IToastNotifier** notifier
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifierForSim2(
                        ABI::Windows::UI::Notifications::IToastNotifier** notifier
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDualSimTileStatics = __uuidof(IDualSimTileStatics);
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.StartScreen.IToastNotificationManagerStatics3
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IToastNotificationManagerStatics3[] = L"Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace Phone {
            namespace StartScreen {
                MIDL_INTERFACE("2717f54b-50df-4455-8e6e-41e0fc8e13ce")
                IToastNotificationManagerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifierForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::IToastNotifier** notifier
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerStatics3 = __uuidof(IToastNotificationManagerStatics3);
            } /* StartScreen */
        } /* Phone */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.StartScreen.DualSimTile
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Phone.StartScreen.DualSimTileContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Phone.StartScreen.IDualSimTileStatics interface starting with version 1.0 of the Windows.Phone.StartScreen.DualSimTileContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Phone.StartScreen.IDualSimTile ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_StartScreen_DualSimTile_DEFINED
#define RUNTIMECLASS_Windows_Phone_StartScreen_DualSimTile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_StartScreen_DualSimTile[] = L"Windows.Phone.StartScreen.DualSimTile";
#endif
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile;

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics;

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3 __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3;

#endif // ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdater __x_ABI_CWindows_CUI_CNotifications_CITileUpdater;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__

/*
 *
 * Interface Windows.Phone.StartScreen.IDualSimTile
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.StartScreen.DualSimTile
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IDualSimTile[] = L"Windows.Phone.StartScreen.IDualSimTile";
typedef struct __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsPinnedToStart)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* UpdateAsync)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileVtbl;

interface __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_get_IsPinnedToStart(This, value) \
    ((This)->lpVtbl->get_IsPinnedToStart(This, value))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_CreateAsync(This, operation) \
    ((This)->lpVtbl->CreateAsync(This, operation))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_UpdateAsync(This, operation) \
    ((This)->lpVtbl->UpdateAsync(This, operation))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_DeleteAsync(This, operation) \
    ((This)->lpVtbl->DeleteAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.StartScreen.IDualSimTileStatics
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Phone.StartScreen.DualSimTile
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IDualSimTileStatics[] = L"Windows.Phone.StartScreen.IDualSimTileStatics";
typedef struct __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTileForSim2)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTile** result);
    HRESULT (STDMETHODCALLTYPE* UpdateDisplayNameForSim1Async)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        HSTRING name,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForSim1)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** updater);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForSim2)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** updater);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForSim1)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** updater);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForSim2)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** updater);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifierForSim1)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** notifier);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifierForSim2)(__x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** notifier);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStaticsVtbl;

interface __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_GetTileForSim2(This, result) \
    ((This)->lpVtbl->GetTileForSim2(This, result))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_UpdateDisplayNameForSim1Async(This, name, operation) \
    ((This)->lpVtbl->UpdateDisplayNameForSim1Async(This, name, operation))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateTileUpdaterForSim1(This, updater) \
    ((This)->lpVtbl->CreateTileUpdaterForSim1(This, updater))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateTileUpdaterForSim2(This, updater) \
    ((This)->lpVtbl->CreateTileUpdaterForSim2(This, updater))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateBadgeUpdaterForSim1(This, updater) \
    ((This)->lpVtbl->CreateBadgeUpdaterForSim1(This, updater))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateBadgeUpdaterForSim2(This, updater) \
    ((This)->lpVtbl->CreateBadgeUpdaterForSim2(This, updater))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateToastNotifierForSim1(This, notifier) \
    ((This)->lpVtbl->CreateToastNotifierForSim1(This, notifier))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_CreateToastNotifierForSim2(This, notifier) \
    ((This)->lpVtbl->CreateToastNotifierForSim2(This, notifier))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIDualSimTileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Phone.StartScreen.IToastNotificationManagerStatics3
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Phone_StartScreen_IToastNotificationManagerStatics3[] = L"Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
typedef struct __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifierForSecondaryTile)(__x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** notifier);

    END_INTERFACE
} __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3Vtbl;

interface __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_CreateToastNotifierForSecondaryTile(This, tileId, notifier) \
    ((This)->lpVtbl->CreateToastNotifierForSecondaryTile(This, tileId, notifier))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CPhone_CStartScreen_CIToastNotificationManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Phone.StartScreen.DualSimTile
 *
 * Introduced to Windows.Phone.StartScreen.DualSimTileContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Phone.StartScreen.DualSimTileContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Phone.StartScreen.IDualSimTileStatics interface starting with version 1.0 of the Windows.Phone.StartScreen.DualSimTileContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Phone.StartScreen.IDualSimTile ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 */
#if WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Phone_StartScreen_DualSimTile_DEFINED
#define RUNTIMECLASS_Windows_Phone_StartScreen_DualSimTile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Phone_StartScreen_DualSimTile[] = L"Windows.Phone.StartScreen.DualSimTile";
#endif
#endif // WINDOWS_PHONE_STARTSCREEN_DUALSIMTILECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Ephone2Estartscreen_p_h__

#endif // __windows2Ephone2Estartscreen_h__
