
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
#ifndef __windows2Eui2Ecomposition2Eeffects_h__
#define __windows2Eui2Ecomposition2Eeffects_h__
#ifndef __windows2Eui2Ecomposition2Eeffects_p_h__
#define __windows2Eui2Ecomposition2Eeffects_p_h__


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
#include "Windows.Graphics.Effects.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    interface ISceneLightingEffect;
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect ABI::Windows::UI::Composition::Effects::ISceneLightingEffect

#endif // ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    interface ISceneLightingEffect2;
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2 ABI::Windows::UI::Composition::Effects::ISceneLightingEffect2

#endif // ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Effects {
                interface IGraphicsEffect;
            } /* Effects */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect ABI::Windows::Graphics::Effects::IGraphicsEffect

#endif // ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Effects {
                interface IGraphicsEffectSource;
            } /* Effects */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource ABI::Windows::Graphics::Effects::IGraphicsEffectSource

#endif // ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    typedef enum SceneLightingEffectReflectanceModel : int SceneLightingEffectReflectanceModel;
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    enum SceneLightingEffectReflectanceModel : int
                    {
                        SceneLightingEffectReflectanceModel_BlinnPhong = 0,
                        SceneLightingEffectReflectanceModel_PhysicallyBasedBlinnPhong = 1,
                    };
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Composition.Effects.ISceneLightingEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Effects.SceneLightingEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Effects_ISceneLightingEffect[] = L"Windows.UI.Composition.Effects.ISceneLightingEffect";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    MIDL_INTERFACE("91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a")
                    ISceneLightingEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AmbientAmount(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AmbientAmount(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DiffuseAmount(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DiffuseAmount(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NormalMapSource(
                            ABI::Windows::Graphics::Effects::IGraphicsEffectSource** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NormalMapSource(
                            ABI::Windows::Graphics::Effects::IGraphicsEffectSource* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SpecularAmount(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SpecularAmount(
                            FLOAT value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SpecularShine(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SpecularShine(
                            FLOAT value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneLightingEffect = __uuidof(ISceneLightingEffect);
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Composition.Effects.ISceneLightingEffect2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Effects.SceneLightingEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Effects_ISceneLightingEffect2[] = L"Windows.UI.Composition.Effects.ISceneLightingEffect2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Composition {
                namespace Effects {
                    MIDL_INTERFACE("9e270e81-72f0-4c5c-95f8-8a6e0024f409")
                    ISceneLightingEffect2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ReflectanceModel(
                            ABI::Windows::UI::Composition::Effects::SceneLightingEffectReflectanceModel* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ReflectanceModel(
                            ABI::Windows::UI::Composition::Effects::SceneLightingEffectReflectanceModel value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISceneLightingEffect2 = __uuidof(ISceneLightingEffect2);
                } /* Effects */
            } /* Composition */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Composition.Effects.SceneLightingEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Effects.ISceneLightingEffect ** Default Interface **
 *    Windows.UI.Composition.Effects.ISceneLightingEffect2
 *    Windows.Graphics.Effects.IGraphicsEffect
 *    Windows.Graphics.Effects.IGraphicsEffectSource
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Effects_SceneLightingEffect_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Effects_SceneLightingEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Effects_SceneLightingEffect[] = L"Windows.UI.Composition.Effects.SceneLightingEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect;

#endif // ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2 __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2;

#endif // ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect;

#endif // ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource;

#endif // ____x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CComposition_CEffects_CSceneLightingEffectReflectanceModel __x_ABI_CWindows_CUI_CComposition_CEffects_CSceneLightingEffectReflectanceModel;

/*
 *
 * Struct Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CUI_CComposition_CEffects_CSceneLightingEffectReflectanceModel
{
    SceneLightingEffectReflectanceModel_BlinnPhong = 0,
    SceneLightingEffectReflectanceModel_PhysicallyBasedBlinnPhong = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Composition.Effects.ISceneLightingEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Effects.SceneLightingEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Effects_ISceneLightingEffect[] = L"Windows.UI.Composition.Effects.ISceneLightingEffect";
typedef struct __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AmbientAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_AmbientAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_DiffuseAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_DiffuseAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_NormalMapSource)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource** value);
    HRESULT (STDMETHODCALLTYPE* put_NormalMapSource)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        __x_ABI_CWindows_CGraphics_CEffects_CIGraphicsEffectSource* value);
    HRESULT (STDMETHODCALLTYPE* get_SpecularAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_SpecularAmount)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT value);
    HRESULT (STDMETHODCALLTYPE* get_SpecularShine)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_SpecularShine)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffectVtbl;

interface __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_get_AmbientAmount(This, value) \
    ((This)->lpVtbl->get_AmbientAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_put_AmbientAmount(This, value) \
    ((This)->lpVtbl->put_AmbientAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_get_DiffuseAmount(This, value) \
    ((This)->lpVtbl->get_DiffuseAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_put_DiffuseAmount(This, value) \
    ((This)->lpVtbl->put_DiffuseAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_get_NormalMapSource(This, value) \
    ((This)->lpVtbl->get_NormalMapSource(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_put_NormalMapSource(This, value) \
    ((This)->lpVtbl->put_NormalMapSource(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_get_SpecularAmount(This, value) \
    ((This)->lpVtbl->get_SpecularAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_put_SpecularAmount(This, value) \
    ((This)->lpVtbl->put_SpecularAmount(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_get_SpecularShine(This, value) \
    ((This)->lpVtbl->get_SpecularShine(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_put_SpecularShine(This, value) \
    ((This)->lpVtbl->put_SpecularShine(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Composition.Effects.ISceneLightingEffect2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Composition.Effects.SceneLightingEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Composition_Effects_ISceneLightingEffect2[] = L"Windows.UI.Composition.Effects.ISceneLightingEffect2";
typedef struct __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReflectanceModel)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        enum __x_ABI_CWindows_CUI_CComposition_CEffects_CSceneLightingEffectReflectanceModel* value);
    HRESULT (STDMETHODCALLTYPE* put_ReflectanceModel)(__x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2* This,
        enum __x_ABI_CWindows_CUI_CComposition_CEffects_CSceneLightingEffectReflectanceModel value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2Vtbl;

interface __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_get_ReflectanceModel(This, value) \
    ((This)->lpVtbl->get_ReflectanceModel(This, value))

#define __x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_put_ReflectanceModel(This, value) \
    ((This)->lpVtbl->put_ReflectanceModel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2;
#endif /* !defined(____x_ABI_CWindows_CUI_CComposition_CEffects_CISceneLightingEffect2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Composition.Effects.SceneLightingEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Composition.Effects.ISceneLightingEffect ** Default Interface **
 *    Windows.UI.Composition.Effects.ISceneLightingEffect2
 *    Windows.Graphics.Effects.IGraphicsEffect
 *    Windows.Graphics.Effects.IGraphicsEffectSource
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Composition_Effects_SceneLightingEffect_DEFINED
#define RUNTIMECLASS_Windows_UI_Composition_Effects_SceneLightingEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Composition_Effects_SceneLightingEffect[] = L"Windows.UI.Composition.Effects.SceneLightingEffect";
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
#endif // __windows2Eui2Ecomposition2Eeffects_p_h__

#endif // __windows2Eui2Ecomposition2Eeffects_h__
