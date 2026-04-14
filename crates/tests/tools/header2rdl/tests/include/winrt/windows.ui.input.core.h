
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
#ifndef __windows2Eui2Einput2Ecore_h__
#define __windows2Eui2Einput2Ecore_h__
#ifndef __windows2Eui2Einput2Ecore_p_h__
#define __windows2Eui2Einput2Ecore_p_h__


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

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Core.h"
#include "Windows.System.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Input.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    interface IRadialControllerIndependentInputSource;
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource ABI::Windows::UI::Input::Core::IRadialControllerIndependentInputSource

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    interface IRadialControllerIndependentInputSource2;
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2 ABI::Windows::UI::Input::Core::IRadialControllerIndependentInputSource2

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    interface IRadialControllerIndependentInputSourceStatics;
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics ABI::Windows::UI::Input::Core::IRadialControllerIndependentInputSourceStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class CoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface ICoreApplicationView;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView ABI::Windows::ApplicationModel::Core::ICoreApplicationView

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace System {
            class DispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IDispatcherQueue;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIDispatcherQueue ABI::Windows::System::IDispatcherQueue

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class CoreDispatcher;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface ICoreDispatcher;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CICoreDispatcher ABI::Windows::UI::Core::ICoreDispatcher

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class RadialController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IRadialController;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIRadialController ABI::Windows::UI::Input::IRadialController

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    class RadialControllerIndependentInputSource;
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSource[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSource";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    MIDL_INTERFACE("3d577ef6-4cee-11e6-b535-001bdc06ab3b")
                    IRadialControllerIndependentInputSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Controller(
                            ABI::Windows::UI::Input::IRadialController** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Dispatcher(
                            ABI::Windows::UI::Core::ICoreDispatcher** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRadialControllerIndependentInputSource = __uuidof(IRadialControllerIndependentInputSource);
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSource2[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSource2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    MIDL_INTERFACE("7073aad8-35f3-4eeb-8751-be4d0a66faf4")
                    IRadialControllerIndependentInputSource2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DispatcherQueue(
                            ABI::Windows::System::IDispatcherQueue** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRadialControllerIndependentInputSource2 = __uuidof(IRadialControllerIndependentInputSource2);
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSourceStatics[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Core {
                    MIDL_INTERFACE("3d577ef5-4cee-11e6-b535-001bdc06ab3b")
                    IRadialControllerIndependentInputSourceStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateForView(
                            ABI::Windows::ApplicationModel::Core::ICoreApplicationView* view,
                            ABI::Windows::UI::Input::Core::IRadialControllerIndependentInputSource** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRadialControllerIndependentInputSourceStatics = __uuidof(IRadialControllerIndependentInputSourceStatics);
                } /* Core */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Core.IRadialControllerIndependentInputSource ** Default Interface **
 *    Windows.UI.Input.Core.IRadialControllerIndependentInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Core_RadialControllerIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Core_RadialControllerIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Core_RadialControllerIndependentInputSource[] = L"Windows.UI.Input.Core.RadialControllerIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource;

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2 __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2;

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIDispatcherQueue __x_ABI_CWindows_CSystem_CIDispatcherQueue;

#endif // ____x_ABI_CWindows_CSystem_CIDispatcherQueue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CICoreDispatcher __x_ABI_CWindows_CUI_CCore_CICoreDispatcher;

#endif // ____x_ABI_CWindows_CUI_CCore_CICoreDispatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIRadialController __x_ABI_CWindows_CUI_CInput_CIRadialController;

#endif // ____x_ABI_CWindows_CUI_CInput_CIRadialController_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSource[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSource";
typedef struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Controller)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        __x_ABI_CWindows_CUI_CInput_CIRadialController** value);
    HRESULT (STDMETHODCALLTYPE* get_Dispatcher)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource* This,
        __x_ABI_CWindows_CUI_CCore_CICoreDispatcher** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceVtbl;

interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_get_Controller(This, value) \
    ((This)->lpVtbl->get_Controller(This, value))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_get_Dispatcher(This, value) \
    ((This)->lpVtbl->get_Dispatcher(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSource2[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSource2";
typedef struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DispatcherQueue)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2* This,
        __x_ABI_CWindows_CSystem_CIDispatcherQueue** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_get_DispatcherQueue(This, value) \
    ((This)->lpVtbl->get_DispatcherQueue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Core_IRadialControllerIndependentInputSourceStatics[] = L"Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForView)(__x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CICoreApplicationView* view,
        __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSource** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_CreateForView(This, view, result) \
    ((This)->lpVtbl->CreateForView(This, view, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CCore_CIRadialControllerIndependentInputSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Core.RadialControllerIndependentInputSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Core.IRadialControllerIndependentInputSource ** Default Interface **
 *    Windows.UI.Input.Core.IRadialControllerIndependentInputSource2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Core_RadialControllerIndependentInputSource_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Core_RadialControllerIndependentInputSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Core_RadialControllerIndependentInputSource[] = L"Windows.UI.Input.Core.RadialControllerIndependentInputSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput2Ecore_p_h__

#endif // __windows2Eui2Einput2Ecore_h__
