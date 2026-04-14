
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
#ifndef __windows2Eui2Einput2Einking2Epreview_h__
#define __windows2Eui2Einput2Einking2Epreview_h__
#ifndef __windows2Eui2Einput2Einking2Epreview_p_h__
#define __windows2Eui2Einput2Einking2Epreview_p_h__


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
#include "Windows.UI.Composition.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Preview {
                        interface IPalmRejectionDelayZonePreview;
                    } /* Preview */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview ABI::Windows::UI::Input::Inking::Preview::IPalmRejectionDelayZonePreview

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Preview {
                        interface IPalmRejectionDelayZonePreviewStatics;
                    } /* Preview */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics ABI::Windows::UI::Input::Inking::Preview::IPalmRejectionDelayZonePreviewStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                class Visual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                interface IVisual;
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CIVisual ABI::Windows::UI::Composition::IVisual

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Preview {
                        class PalmRejectionDelayZonePreview;
                    } /* Preview */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Preview_IPalmRejectionDelayZonePreview[] = L"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Preview {
                        MIDL_INTERFACE("62b496cb-539d-5343-a65f-41f5300ec70c")
                        IPalmRejectionDelayZonePreview : public IInspectable
                        {
                        public:
                        };

                        MIDL_CONST_ID IID& IID_IPalmRejectionDelayZonePreview = __uuidof(IPalmRejectionDelayZonePreview);
                    } /* Preview */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Preview_IPalmRejectionDelayZonePreviewStatics[] = L"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    namespace Preview {
                        MIDL_INTERFACE("cdef5ee0-93d0-53a9-8f0e-9a379f8f7530")
                        IPalmRejectionDelayZonePreviewStatics : public IInspectable
                        {
                        public:
                            virtual HRESULT STDMETHODCALLTYPE CreateForVisual(
                                ABI::Windows::UI::Composition::IVisual* inputPanelVisual,
                                ABI::Windows::Foundation::Rect inputPanelRect,
                                ABI::Windows::UI::Input::Inking::Preview::IPalmRejectionDelayZonePreview** result
                                ) = 0;
                            virtual HRESULT STDMETHODCALLTYPE CreateForVisualWithViewportClip(
                                ABI::Windows::UI::Composition::IVisual* inputPanelVisual,
                                ABI::Windows::Foundation::Rect inputPanelRect,
                                ABI::Windows::UI::Composition::IVisual* viewportVisual,
                                ABI::Windows::Foundation::Rect viewportRect,
                                ABI::Windows::UI::Input::Inking::Preview::IPalmRejectionDelayZonePreview** result
                                ) = 0;
                        };

                        MIDL_CONST_ID IID& IID_IPalmRejectionDelayZonePreviewStatics = __uuidof(IPalmRejectionDelayZonePreviewStatics);
                    } /* Preview */
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview[] = L"Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CIVisual __x_ABI_CWindows_CUI_CComposition_CIVisual;

#endif // ____x_ABI_CWindows_CUI_CComposition_CIVisual_FWD_DEFINED__

/*
 *
 * Interface Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Preview_IPalmRejectionDelayZonePreview[] = L"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_Preview_IPalmRejectionDelayZonePreviewStatics[] = L"Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForVisual)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* inputPanelVisual,
        struct __x_ABI_CWindows_CFoundation_CRect inputPanelRect,
        __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview** result);
    HRESULT (STDMETHODCALLTYPE* CreateForVisualWithViewportClip)(__x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics* This,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* inputPanelVisual,
        struct __x_ABI_CWindows_CFoundation_CRect inputPanelRect,
        __x_ABI_CWindows_CUI_CComposition_CIVisual* viewportVisual,
        struct __x_ABI_CWindows_CFoundation_CRect viewportRect,
        __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_CreateForVisual(This, inputPanelVisual, inputPanelRect, result) \
    ((This)->lpVtbl->CreateForVisual(This, inputPanelVisual, inputPanelRect, result))

#define __x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_CreateForVisualWithViewportClip(This, inputPanelVisual, inputPanelRect, viewportVisual, viewportRect, result) \
    ((This)->lpVtbl->CreateForVisualWithViewportClip(This, inputPanelVisual, inputPanelRect, viewportVisual, viewportRect, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CPreview_CIPalmRejectionDelayZonePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreviewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.Preview.IPalmRejectionDelayZonePreview ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_Preview_PalmRejectionDelayZonePreview[] = L"Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
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
#endif // __windows2Eui2Einput2Einking2Epreview_p_h__

#endif // __windows2Eui2Einput2Einking2Epreview_h__
