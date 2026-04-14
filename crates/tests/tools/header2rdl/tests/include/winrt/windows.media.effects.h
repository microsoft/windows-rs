
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
#ifndef __windows2Emedia2Eeffects_h__
#define __windows2Emedia2Eeffects_h__
#ifndef __windows2Emedia2Eeffects_p_h__
#define __windows2Emedia2Eeffects_p_h__


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

#if !defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPBROADCASTCONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTURECONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_APPCAPTUREMETADATACONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_CAMERACAPTUREUICONTRACT_VERSION)

#if !defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)
#define WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_CAPTURE_GAMEBARCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Foundation.Numerics.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Media.h"
#include "Windows.Media.Capture.h"
#include "Windows.Media.Editing.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Media.Playback.h"
#include "Windows.Media.Render.h"
#include "Windows.Media.Transcoding.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAcousticEchoCancellationConfiguration;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration ABI::Windows::Media::Effects::IAcousticEchoCancellationConfiguration

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioCaptureEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager ABI::Windows::Media::Effects::IAudioCaptureEffectsManager

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffect;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect ABI::Windows::Media::Effects::IAudioEffect

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffect2;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2 ABI::Windows::Media::Effects::IAudioEffect2

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition ABI::Windows::Media::Effects::IAudioEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffectDefinitionFactory;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory ABI::Windows::Media::Effects::IAudioEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffectsManagerStatics;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics ABI::Windows::Media::Effects::IAudioEffectsManagerStatics

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioRenderEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager ABI::Windows::Media::Effects::IAudioRenderEffectsManager

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioRenderEffectsManager2;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2 ABI::Windows::Media::Effects::IAudioRenderEffectsManager2

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IBasicAudioEffect;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect ABI::Windows::Media::Effects::IBasicAudioEffect

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IBasicVideoEffect;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect ABI::Windows::Media::Effects::IBasicVideoEffect

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface ICompositeVideoFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext ABI::Windows::Media::Effects::ICompositeVideoFrameContext

#endif // ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IProcessAudioFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext ABI::Windows::Media::Effects::IProcessAudioFrameContext

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IProcessVideoFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext ABI::Windows::Media::Effects::IProcessVideoFrameContext

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoCompositor;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor ABI::Windows::Media::Effects::IVideoCompositor

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoCompositorDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition ABI::Windows::Media::Effects::IVideoCompositorDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoCompositorDefinitionFactory;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory ABI::Windows::Media::Effects::IVideoCompositorDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition ABI::Windows::Media::Effects::IVideoEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoEffectDefinitionFactory;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory ABI::Windows::Media::Effects::IVideoEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoTransformEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition ABI::Windows::Media::Effects::IVideoTransformEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoTransformEffectDefinition2;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2 ABI::Windows::Media::Effects::IVideoTransformEffectDefinition2

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IVideoTransformSphericalProjection;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection ABI::Windows::Media::Effects::IVideoTransformSphericalProjection

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DSurface;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#define DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bdfb6d0b-e785-5d5a-abd2-fe1b18c43257"))
IIterator<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> : IIterator_impl<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t;
#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#define DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cc63bf9c-e16a-5a75-a5aa-2b53f975b0b0"))
IIterable<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> : IIterable_impl<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t;
#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AudioEffect;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_USE
#define DEF___FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("673ce717-a3cf-5d68-a80b-5ed3e7b93fed"))
IIterator<ABI::Windows::Media::Effects::AudioEffect*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Effects::AudioEffect*, ABI::Windows::Media::Effects::IAudioEffect*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Effects.AudioEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Effects::AudioEffect*> __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_t;
#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_USE
#define DEF___FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("07af9afd-25b8-579d-be7e-8acc03418d0b"))
IIterable<ABI::Windows::Media::Effects::AudioEffect*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Effects::AudioEffect*, ABI::Windows::Media::Effects::IAudioEffect*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Effects.AudioEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Effects::AudioEffect*> __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_t;
#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class AudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties ABI::Windows::Media::MediaProperties::IAudioEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#define DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("064b6aef-3bd0-5463-8450-72c6169af5d4"))
IIterator<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*, ABI::Windows::Media::MediaProperties::IAudioEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.MediaProperties.AudioEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t;
#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#define DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("00939468-25d8-533f-854e-4f20f36c51dc"))
IIterable<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*, ABI::Windows::Media::MediaProperties::IAudioEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.MediaProperties.AudioEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t;
#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class VideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties ABI::Windows::Media::MediaProperties::IVideoEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#define DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("094166ef-ae5f-5315-a3bf-fe54e8c35fcd"))
IIterator<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*, ABI::Windows::Media::MediaProperties::IVideoEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.MediaProperties.VideoEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t;
#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#define DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("73c7317c-8682-5f81-84a2-30c425fa2d24"))
IIterable<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*, ABI::Windows::Media::MediaProperties::IVideoEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.MediaProperties.VideoEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t;
#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1a81ec3e-5afb-5e10-92bb-c843fec70887"))
IVectorView<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> : IVectorView_impl<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface*> __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t;
#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cdcbc9e7-53d8-5e66-9e45-31d5a23fd01d"))
IVectorView<ABI::Windows::Media::Effects::AudioEffect*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Effects::AudioEffect*, ABI::Windows::Media::Effects::IAudioEffect*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Effects.AudioEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Effects::AudioEffect*> __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_t;
#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#define DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("64ac506c-ccd0-56cb-b088-b1a36e8755df"))
IVectorView<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*, ABI::Windows::Media::MediaProperties::IAudioEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.MediaProperties.AudioEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::MediaProperties::AudioEncodingProperties*> __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t;
#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#define DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("04c7baba-04d7-51db-a4e3-5147963ec5ff"))
IVectorView<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*, ABI::Windows::Media::MediaProperties::IVideoEncodingProperties*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.MediaProperties.VideoEncodingProperties>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::MediaProperties::VideoEncodingProperties*> __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t;
#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AudioCaptureEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a1c5e803-a275-5bb1-9d44-2ac8ae9ffb89"))
ITypedEventHandler<ABI::Windows::Media::Effects::AudioCaptureEffectsManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Effects::AudioCaptureEffectsManager*, ABI::Windows::Media::Effects::IAudioCaptureEffectsManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Effects.AudioCaptureEffectsManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Effects::AudioCaptureEffectsManager*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AudioRenderEffectsManager;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4be29c7e-449c-576e-a7b8-3a40f2f01dc8"))
ITypedEventHandler<ABI::Windows::Media::Effects::AudioRenderEffectsManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Effects::AudioRenderEffectsManager*, ABI::Windows::Media::Effects::IAudioRenderEffectsManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Effects.AudioRenderEffectsManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Effects::AudioRenderEffectsManager*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DDevice;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            class AudioFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IAudioFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIAudioFrame ABI::Windows::Media::IAudioFrame

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum AudioProcessing : int AudioProcessing;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                typedef enum MediaCategory : int MediaCategory;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                class MediaOverlay;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Editing {
                interface IMediaOverlay;
            } /* Editing */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay ABI::Windows::Media::Editing::IMediaOverlay

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaExtension;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaExtension ABI::Windows::Media::IMediaExtension

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum MediaMirroringOptions : unsigned int MediaMirroringOptions;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum MediaRotation : int MediaRotation;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum SphericalVideoFrameFormat : int SphericalVideoFrameFormat;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Playback {
                typedef enum SphericalVideoProjectionMode : int SphericalVideoProjectionMode;
            } /* Playback */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Render {
                typedef enum AudioRenderCategory : int AudioRenderCategory;
            } /* Render */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                typedef enum MediaVideoProcessingAlgorithm : int MediaVideoProcessingAlgorithm;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class VideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrame ABI::Windows::Media::IVideoFrame

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                typedef enum AudioEffectState : int AudioEffectState;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                typedef enum AudioEffectType : int AudioEffectType;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                typedef enum MediaEffectClosedReason : int MediaEffectClosedReason;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                typedef enum MediaMemoryTypes : int MediaMemoryTypes;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AcousticEchoCancellationConfiguration;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class AudioEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class CompositeVideoFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class ProcessAudioFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class ProcessVideoFrameContext;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class VideoCompositorDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class VideoEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                class VideoTransformSphericalProjection;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Effects.AudioEffectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                enum AudioEffectState : int
                {
                    AudioEffectState_Off = 0,
                    AudioEffectState_On = 1,
                };
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Media.Effects.AudioEffectType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                enum AudioEffectType : int
                {
                    AudioEffectType_Other = 0,
                    AudioEffectType_AcousticEchoCancellation = 1,
                    AudioEffectType_NoiseSuppression = 2,
                    AudioEffectType_AutomaticGainControl = 3,
                    AudioEffectType_BeamForming = 4,
                    AudioEffectType_ConstantToneRemoval = 5,
                    AudioEffectType_Equalizer = 6,
                    AudioEffectType_LoudnessEqualizer = 7,
                    AudioEffectType_BassBoost = 8,
                    AudioEffectType_VirtualSurround = 9,
                    AudioEffectType_VirtualHeadphones = 10,
                    AudioEffectType_SpeakerFill = 11,
                    AudioEffectType_RoomCorrection = 12,
                    AudioEffectType_BassManagement = 13,
                    AudioEffectType_EnvironmentalEffects = 14,
                    AudioEffectType_SpeakerProtection = 15,
                    AudioEffectType_SpeakerCompensation = 16,
                    AudioEffectType_DynamicRangeCompression = 17,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    AudioEffectType_FarFieldBeamForming = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                    AudioEffectType_DeepNoiseSuppression = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
                };
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Effects.MediaEffectClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                enum MediaEffectClosedReason : int
                {
                    MediaEffectClosedReason_Done = 0,
                    MediaEffectClosedReason_UnknownError = 1,
                    MediaEffectClosedReason_UnsupportedEncodingFormat = 2,
                    MediaEffectClosedReason_EffectCurrentlyUnloaded = 3,
                };
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Effects.MediaMemoryTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                enum MediaMemoryTypes : int
                {
                    MediaMemoryTypes_Gpu = 0,
                    MediaMemoryTypes_Cpu = 1,
                    MediaMemoryTypes_GpuAndCpu = 2,
                };
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAcousticEchoCancellationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AcousticEchoCancellationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAcousticEchoCancellationConfiguration[] = L"Windows.Media.Effects.IAcousticEchoCancellationConfiguration";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("587e735b-175b-5177-a407-2e33bafe33a5")
                IAcousticEchoCancellationConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetEchoCancellationRenderEndpoint(
                        HSTRING deviceId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAcousticEchoCancellationConfiguration = __uuidof(IAcousticEchoCancellationConfiguration);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Effects.IAudioCaptureEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioCaptureEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioCaptureEffectsManager[] = L"Windows.Media.Effects.IAudioCaptureEffectsManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("8f85c271-038d-4393-8298-540110608eef")
                IAudioCaptureEffectsManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AudioCaptureEffectsChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioCaptureEffectsChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioCaptureEffects(
                        __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect** effects
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioCaptureEffectsManager = __uuidof(IAudioCaptureEffectsManager);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffect[] = L"Windows.Media.Effects.IAudioEffect";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("34aafa51-9207-4055-be93-6e5734a86ae4")
                IAudioEffect : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AudioEffectType(
                        ABI::Windows::Media::Effects::AudioEffectType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffect = __uuidof(IAudioEffect);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffect2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffect2[] = L"Windows.Media.Effects.IAudioEffect2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("06703cb0-757e-5757-8af0-6ba58a8b2990")
                IAudioEffect2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AcousticEchoCancellationConfiguration(
                        ABI::Windows::Media::Effects::IAcousticEchoCancellationConfiguration** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanSetState(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::Effects::AudioEffectState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetState(
                        ABI::Windows::Media::Effects::AudioEffectState newState
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffect2 = __uuidof(IAudioEffect2);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectDefinition[] = L"Windows.Media.Effects.IAudioEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("e4d7f974-7d80-4f73-9089-e31c9db9c294")
                IAudioEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatableClassId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffectDefinition = __uuidof(IAudioEffectDefinition);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectDefinitionFactory[] = L"Windows.Media.Effects.IAudioEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("8e1da646-e705-45ed-8a2b-fc4e4f405a97")
                IAudioEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING activatableClassId,
                        ABI::Windows::Media::Effects::IAudioEffectDefinition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithProperties(
                        HSTRING activatableClassId,
                        ABI::Windows::Foundation::Collections::IPropertySet* props,
                        ABI::Windows::Media::Effects::IAudioEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffectDefinitionFactory = __uuidof(IAudioEffectDefinitionFactory);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectsManagerStatics[] = L"Windows.Media.Effects.IAudioEffectsManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("66406c04-86fa-47cc-a315-f489d8c3fe10")
                IAudioEffectsManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioRenderEffectsManager(
                        HSTRING deviceId,
                        ABI::Windows::Media::Render::AudioRenderCategory category,
                        ABI::Windows::Media::Effects::IAudioRenderEffectsManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioRenderEffectsManagerWithMode(
                        HSTRING deviceId,
                        ABI::Windows::Media::Render::AudioRenderCategory category,
                        ABI::Windows::Media::AudioProcessing mode,
                        ABI::Windows::Media::Effects::IAudioRenderEffectsManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioCaptureEffectsManager(
                        HSTRING deviceId,
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::Effects::IAudioCaptureEffectsManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioCaptureEffectsManagerWithMode(
                        HSTRING deviceId,
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::AudioProcessing mode,
                        ABI::Windows::Media::Effects::IAudioCaptureEffectsManager** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffectsManagerStatics = __uuidof(IAudioEffectsManagerStatics);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioRenderEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioRenderEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioRenderEffectsManager[] = L"Windows.Media.Effects.IAudioRenderEffectsManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("4dc98966-8751-42b2-bfcb-39ca7864bd47")
                IAudioRenderEffectsManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_AudioRenderEffectsChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioRenderEffectsChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioRenderEffects(
                        __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect** effects
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioRenderEffectsManager = __uuidof(IAudioRenderEffectsManager);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioRenderEffectsManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioRenderEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioRenderEffectsManager2[] = L"Windows.Media.Effects.IAudioRenderEffectsManager2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("a844cd09-5ecc-44b3-bb4e-1db07287139c")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IAudioRenderEffectsManager2 : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_EffectsProviderThumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_EffectsProviderSettingsLabel(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ShowSettingsUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioRenderEffectsManager2 = __uuidof(IAudioRenderEffectsManager2);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IBasicAudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IBasicAudioEffect[] = L"Windows.Media.Effects.IBasicAudioEffect";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("8c062c53-6bc0-48b8-a99a-4b41550f1359")
                IBasicAudioEffect : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UseInputFrameForOutput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedEncodingProperties(
                        __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessFrame(
                        ABI::Windows::Media::Effects::IProcessAudioFrameContext* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Close(
                        ABI::Windows::Media::Effects::MediaEffectClosedReason reason
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DiscardQueuedFrames(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IBasicAudioEffect = __uuidof(IBasicAudioEffect);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IBasicVideoEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IBasicVideoEffect[] = L"Windows.Media.Effects.IBasicVideoEffect";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("8262c7ef-b360-40be-949b-2ff42ff35693")
                IBasicVideoEffect : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedMemoryTypes(
                        ABI::Windows::Media::Effects::MediaMemoryTypes* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TimeIndependent(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedEncodingProperties(
                        __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties* encodingProperties,
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ProcessFrame(
                        ABI::Windows::Media::Effects::IProcessVideoFrameContext* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Close(
                        ABI::Windows::Media::Effects::MediaEffectClosedReason reason
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DiscardQueuedFrames(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IBasicVideoEffect = __uuidof(IBasicVideoEffect);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.ICompositeVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.CompositeVideoFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_ICompositeVideoFrameContext[] = L"Windows.Media.Effects.ICompositeVideoFrameContext";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("6c30024b-f514-4278-a5f7-b9188049d110")
                ICompositeVideoFrameContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SurfacesToOverlay(
                        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BackgroundFrame(
                        ABI::Windows::Media::IVideoFrame** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputFrame(
                        ABI::Windows::Media::IVideoFrame** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOverlayForSurface(
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* surfaceToOverlay,
                        ABI::Windows::Media::Editing::IMediaOverlay** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICompositeVideoFrameContext = __uuidof(ICompositeVideoFrameContext);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IProcessAudioFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.ProcessAudioFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IProcessAudioFrameContext[] = L"Windows.Media.Effects.IProcessAudioFrameContext";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("4cd92946-1222-4a27-a586-fb3e20273255")
                IProcessAudioFrameContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InputFrame(
                        ABI::Windows::Media::IAudioFrame** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputFrame(
                        ABI::Windows::Media::IAudioFrame** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessAudioFrameContext = __uuidof(IProcessAudioFrameContext);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IProcessVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.ProcessVideoFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IProcessVideoFrameContext[] = L"Windows.Media.Effects.IProcessVideoFrameContext";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("276f0e2b-6461-401e-ba78-0fdad6114eec")
                IProcessVideoFrameContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InputFrame(
                        ABI::Windows::Media::IVideoFrame** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputFrame(
                        ABI::Windows::Media::IVideoFrame** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IProcessVideoFrameContext = __uuidof(IProcessVideoFrameContext);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositor[] = L"Windows.Media.Effects.IVideoCompositor";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("8510b43e-420c-420f-96c7-7c98bba1fc55")
                IVideoCompositor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TimeIndependent(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetEncodingProperties(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties* backgroundProperties,
                        ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CompositeFrame(
                        ABI::Windows::Media::Effects::ICompositeVideoFrameContext* context
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Close(
                        ABI::Windows::Media::Effects::MediaEffectClosedReason reason
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DiscardQueuedFrames(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoCompositor = __uuidof(IVideoCompositor);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositorDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositorDefinition[] = L"Windows.Media.Effects.IVideoCompositorDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("7946b8d0-2010-4ae3-9ab2-2cef42edd4d2")
                IVideoCompositorDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatableClassId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoCompositorDefinition = __uuidof(IVideoCompositorDefinition);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositorDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoCompositorDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositorDefinitionFactory[] = L"Windows.Media.Effects.IVideoCompositorDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("4366fd10-68b8-4d52-89b6-02a968cca899")
                IVideoCompositorDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING activatableClassId,
                        ABI::Windows::Media::Effects::IVideoCompositorDefinition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithProperties(
                        HSTRING activatableClassId,
                        ABI::Windows::Foundation::Collections::IPropertySet* props,
                        ABI::Windows::Media::Effects::IVideoCompositorDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoCompositorDefinitionFactory = __uuidof(IVideoCompositorDefinitionFactory);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoEffectDefinition[] = L"Windows.Media.Effects.IVideoEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("39f38cf0-8d0f-4f3e-84fc-2d46a5297943")
                IVideoEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActivatableClassId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEffectDefinition = __uuidof(IVideoEffectDefinition);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoEffectDefinitionFactory[] = L"Windows.Media.Effects.IVideoEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("81439b4e-6e33-428f-9d21-b5aafef7617c")
                IVideoEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING activatableClassId,
                        ABI::Windows::Media::Effects::IVideoEffectDefinition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithProperties(
                        HSTRING activatableClassId,
                        ABI::Windows::Foundation::Collections::IPropertySet* props,
                        ABI::Windows::Media::Effects::IVideoEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEffectDefinitionFactory = __uuidof(IVideoEffectDefinitionFactory);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IVideoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformEffectDefinition[] = L"Windows.Media.Effects.IVideoTransformEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("9664bb6a-1ea6-4aa6-8074-abe8851ecae2")
                IVideoTransformEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PaddingColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PaddingColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutputSize(
                        ABI::Windows::Foundation::Size* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutputSize(
                        ABI::Windows::Foundation::Size value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CropRectangle(
                        ABI::Windows::Foundation::Rect* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CropRectangle(
                        ABI::Windows::Foundation::Rect value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rotation(
                        ABI::Windows::Media::MediaProperties::MediaRotation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Rotation(
                        ABI::Windows::Media::MediaProperties::MediaRotation value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mirror(
                        ABI::Windows::Media::MediaProperties::MediaMirroringOptions* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Mirror(
                        ABI::Windows::Media::MediaProperties::MediaMirroringOptions value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProcessingAlgorithm(
                        ABI::Windows::Media::Transcoding::MediaVideoProcessingAlgorithm value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessingAlgorithm(
                        ABI::Windows::Media::Transcoding::MediaVideoProcessingAlgorithm* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoTransformEffectDefinition = __uuidof(IVideoTransformEffectDefinition);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformEffectDefinition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformEffectDefinition2[] = L"Windows.Media.Effects.IVideoTransformEffectDefinition2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("f0a8089f-66c8-4694-9fd9-1136abf7444a")
                IVideoTransformEffectDefinition2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SphericalProjection(
                        ABI::Windows::Media::Effects::IVideoTransformSphericalProjection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoTransformEffectDefinition2 = __uuidof(IVideoTransformEffectDefinition2);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformSphericalProjection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformSphericalProjection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformSphericalProjection[] = L"Windows.Media.Effects.IVideoTransformSphericalProjection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                MIDL_INTERFACE("cf4401f0-9bf2-4c39-9f41-e022514a8468")
                IVideoTransformSphericalProjection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrameFormat(
                        ABI::Windows::Media::MediaProperties::SphericalVideoFrameFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FrameFormat(
                        ABI::Windows::Media::MediaProperties::SphericalVideoFrameFormat value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProjectionMode(
                        ABI::Windows::Media::Playback::SphericalVideoProjectionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProjectionMode(
                        ABI::Windows::Media::Playback::SphericalVideoProjectionMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HorizontalFieldOfViewInDegrees(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HorizontalFieldOfViewInDegrees(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ViewOrientation(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ViewOrientation(
                        ABI::Windows::Foundation::Numerics::Quaternion value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoTransformSphericalProjection = __uuidof(IVideoTransformSphericalProjection);
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Effects.AcousticEchoCancellationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAcousticEchoCancellationConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AcousticEchoCancellationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AcousticEchoCancellationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AcousticEchoCancellationConfiguration[] = L"Windows.Media.Effects.AcousticEchoCancellationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Media.Effects.AudioCaptureEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioCaptureEffectsManager ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioCaptureEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioCaptureEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioCaptureEffectsManager[] = L"Windows.Media.Effects.AudioCaptureEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioEffect ** Default Interface **
 *    Windows.Media.Effects.IAudioEffect2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffect_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffect[] = L"Windows.Media.Effects.AudioEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IAudioEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioEffectDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffectDefinition[] = L"Windows.Media.Effects.AudioEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Effects.IAudioEffectsManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffectsManager[] = L"Windows.Media.Effects.AudioEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioRenderEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioRenderEffectsManager ** Default Interface **
 *    Windows.Media.Effects.IAudioRenderEffectsManager2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioRenderEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioRenderEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioRenderEffectsManager[] = L"Windows.Media.Effects.AudioRenderEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.CompositeVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.ICompositeVideoFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_CompositeVideoFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_CompositeVideoFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_CompositeVideoFrameContext[] = L"Windows.Media.Effects.CompositeVideoFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.ProcessAudioFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IProcessAudioFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_ProcessAudioFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_ProcessAudioFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_ProcessAudioFrameContext[] = L"Windows.Media.Effects.ProcessAudioFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.ProcessVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IProcessVideoFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_ProcessVideoFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_ProcessVideoFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_ProcessVideoFrameContext[] = L"Windows.Media.Effects.ProcessVideoFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoCompositorDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IVideoCompositorDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoCompositorDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoCompositorDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoCompositorDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoCompositorDefinition[] = L"Windows.Media.Effects.VideoCompositorDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IVideoEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoEffectDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoEffectDefinition[] = L"Windows.Media.Effects.VideoEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoTransformEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IVideoTransformEffectDefinition
 *    Windows.Media.Effects.IVideoTransformEffectDefinition2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoTransformEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoTransformEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoTransformEffectDefinition[] = L"Windows.Media.Effects.VideoTransformEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoTransformSphericalProjection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoTransformSphericalProjection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoTransformSphericalProjection_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoTransformSphericalProjection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoTransformSphericalProjection[] = L"Windows.Media.Effects.VideoTransformSphericalProjection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2 __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2 __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2 __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

typedef struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl;

interface __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

typedef struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        __FIIterator_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl;

interface __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEffects__CAudioEffect;

typedef struct __FIIterator_1_Windows__CMedia__CEffects__CAudioEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEffects__CAudioEffect* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEffects__CAudioEffectVtbl;

interface __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEffects__CAudioEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEffects__CAudioEffect;

typedef struct __FIIterable_1_Windows__CMedia__CEffects__CAudioEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEffects__CAudioEffect* This,
        __FIIterator_1_Windows__CMedia__CEffects__CAudioEffect** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEffects__CAudioEffectVtbl;

interface __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEffects__CAudioEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

typedef struct __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl;

interface __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

typedef struct __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        __FIIterator_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl;

interface __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

typedef struct __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl;

interface __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

typedef struct __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        __FIIterator_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl;

interface __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface;

typedef struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl;

interface __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurfaceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect;

typedef struct __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffectVtbl;

interface __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties;

typedef struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl;

interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties;

typedef struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl;

interface __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioFrame __x_ABI_CWindows_CMedia_CIAudioFrame;

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CAudioProcessing __x_ABI_CWindows_CMedia_CAudioProcessing;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory __x_ABI_CWindows_CMedia_CCapture_CMediaCategory;

#ifndef ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay;

#endif // ____x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaExtension __x_ABI_CWindows_CMedia_CIMediaExtension;

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaMirroringOptions __x_ABI_CWindows_CMedia_CMediaProperties_CMediaMirroringOptions;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaRotation __x_ABI_CWindows_CMedia_CMediaProperties_CMediaRotation;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat;

typedef enum __x_ABI_CWindows_CMedia_CPlayback_CSphericalVideoProjectionMode __x_ABI_CWindows_CMedia_CPlayback_CSphericalVideoProjectionMode;

typedef enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory;

typedef enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm;

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectState __x_ABI_CWindows_CMedia_CEffects_CAudioEffectState;

typedef enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectType __x_ABI_CWindows_CMedia_CEffects_CAudioEffectType;

typedef enum __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason;

typedef enum __x_ABI_CWindows_CMedia_CEffects_CMediaMemoryTypes __x_ABI_CWindows_CMedia_CEffects_CMediaMemoryTypes;

/*
 *
 * Struct Windows.Media.Effects.AudioEffectState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectState
{
    AudioEffectState_Off = 0,
    AudioEffectState_On = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Struct Windows.Media.Effects.AudioEffectType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectType
{
    AudioEffectType_Other = 0,
    AudioEffectType_AcousticEchoCancellation = 1,
    AudioEffectType_NoiseSuppression = 2,
    AudioEffectType_AutomaticGainControl = 3,
    AudioEffectType_BeamForming = 4,
    AudioEffectType_ConstantToneRemoval = 5,
    AudioEffectType_Equalizer = 6,
    AudioEffectType_LoudnessEqualizer = 7,
    AudioEffectType_BassBoost = 8,
    AudioEffectType_VirtualSurround = 9,
    AudioEffectType_VirtualHeadphones = 10,
    AudioEffectType_SpeakerFill = 11,
    AudioEffectType_RoomCorrection = 12,
    AudioEffectType_BassManagement = 13,
    AudioEffectType_EnvironmentalEffects = 14,
    AudioEffectType_SpeakerProtection = 15,
    AudioEffectType_SpeakerCompensation = 16,
    AudioEffectType_DynamicRangeCompression = 17,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    AudioEffectType_FarFieldBeamForming = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
    AudioEffectType_DeepNoiseSuppression = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Effects.MediaEffectClosedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason
{
    MediaEffectClosedReason_Done = 0,
    MediaEffectClosedReason_UnknownError = 1,
    MediaEffectClosedReason_UnsupportedEncodingFormat = 2,
    MediaEffectClosedReason_EffectCurrentlyUnloaded = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Effects.MediaMemoryTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CEffects_CMediaMemoryTypes
{
    MediaMemoryTypes_Gpu = 0,
    MediaMemoryTypes_Cpu = 1,
    MediaMemoryTypes_GpuAndCpu = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAcousticEchoCancellationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AcousticEchoCancellationConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAcousticEchoCancellationConfiguration[] = L"Windows.Media.Effects.IAcousticEchoCancellationConfiguration";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetEchoCancellationRenderEndpoint)(__x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration* This,
        HSTRING deviceId);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfigurationVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_SetEchoCancellationRenderEndpoint(This, deviceId) \
    ((This)->lpVtbl->SetEchoCancellationRenderEndpoint(This, deviceId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Effects.IAudioCaptureEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioCaptureEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioCaptureEffectsManager[] = L"Windows.Media.Effects.IAudioCaptureEffectsManager";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AudioCaptureEffectsChanged)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioCaptureEffectsManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioCaptureEffectsChanged)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetAudioCaptureEffects)(__x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager* This,
        __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect** effects);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManagerVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_add_AudioCaptureEffectsChanged(This, handler, token) \
    ((This)->lpVtbl->add_AudioCaptureEffectsChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_remove_AudioCaptureEffectsChanged(This, token) \
    ((This)->lpVtbl->remove_AudioCaptureEffectsChanged(This, token))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_GetAudioCaptureEffects(This, effects) \
    ((This)->lpVtbl->GetAudioCaptureEffects(This, effects))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffect[] = L"Windows.Media.Effects.IAudioEffect";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioEffectType)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectType* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_get_AudioEffectType(This, value) \
    ((This)->lpVtbl->get_AudioEffectType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffect2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffect2[] = L"Windows.Media.Effects.IAudioEffect2";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcousticEchoCancellationConfiguration)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAcousticEchoCancellationConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* get_CanSetState)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectState* value);
    HRESULT (STDMETHODCALLTYPE* SetState)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CAudioEffectState newState);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2Vtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_get_AcousticEchoCancellationConfiguration(This, value) \
    ((This)->lpVtbl->get_AcousticEchoCancellationConfiguration(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_get_CanSetState(This, value) \
    ((This)->lpVtbl->get_CanSetState(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_SetState(This, newState) \
    ((This)->lpVtbl->SetState(This, newState))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffect2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectDefinition[] = L"Windows.Media.Effects.IAudioEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivatableClassId)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_get_ActivatableClassId(This, value) \
    ((This)->lpVtbl->get_ActivatableClassId(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectDefinitionFactory[] = L"Windows.Media.Effects.IAudioEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithProperties)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* props,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_Create(This, activatableClassId, value) \
    ((This)->lpVtbl->Create(This, activatableClassId, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_CreateWithProperties(This, activatableClassId, props, value) \
    ((This)->lpVtbl->CreateWithProperties(This, activatableClassId, props, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioEffectsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioEffectsManagerStatics[] = L"Windows.Media.Effects.IAudioEffectsManagerStatics";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAudioRenderEffectsManager)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory category,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager** value);
    HRESULT (STDMETHODCALLTYPE* CreateAudioRenderEffectsManagerWithMode)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory category,
        enum __x_ABI_CWindows_CMedia_CAudioProcessing mode,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager** value);
    HRESULT (STDMETHODCALLTYPE* CreateAudioCaptureEffectsManager)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager** value);
    HRESULT (STDMETHODCALLTYPE* CreateAudioCaptureEffectsManagerWithMode)(__x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics* This,
        HSTRING deviceId,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        enum __x_ABI_CWindows_CMedia_CAudioProcessing mode,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioCaptureEffectsManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_CreateAudioRenderEffectsManager(This, deviceId, category, value) \
    ((This)->lpVtbl->CreateAudioRenderEffectsManager(This, deviceId, category, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_CreateAudioRenderEffectsManagerWithMode(This, deviceId, category, mode, value) \
    ((This)->lpVtbl->CreateAudioRenderEffectsManagerWithMode(This, deviceId, category, mode, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_CreateAudioCaptureEffectsManager(This, deviceId, category, value) \
    ((This)->lpVtbl->CreateAudioCaptureEffectsManager(This, deviceId, category, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_CreateAudioCaptureEffectsManagerWithMode(This, deviceId, category, mode, value) \
    ((This)->lpVtbl->CreateAudioCaptureEffectsManagerWithMode(This, deviceId, category, mode, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioRenderEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioRenderEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioRenderEffectsManager[] = L"Windows.Media.Effects.IAudioRenderEffectsManager";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_AudioRenderEffectsChanged)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        __FITypedEventHandler_2_Windows__CMedia__CEffects__CAudioRenderEffectsManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioRenderEffectsChanged)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetAudioRenderEffects)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager* This,
        __FIVectorView_1_Windows__CMedia__CEffects__CAudioEffect** effects);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManagerVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_add_AudioRenderEffectsChanged(This, handler, token) \
    ((This)->lpVtbl->add_AudioRenderEffectsChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_remove_AudioRenderEffectsChanged(This, token) \
    ((This)->lpVtbl->remove_AudioRenderEffectsChanged(This, token))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_GetAudioRenderEffects(This, effects) \
    ((This)->lpVtbl->GetAudioRenderEffects(This, effects))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IAudioRenderEffectsManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.AudioRenderEffectsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IAudioRenderEffectsManager2[] = L"Windows.Media.Effects.IAudioRenderEffectsManager2";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_EffectsProviderThumbnail)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_EffectsProviderSettingsLabel)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ShowSettingsUI)(__x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2Vtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_get_EffectsProviderThumbnail(This, value) \
    ((This)->lpVtbl->get_EffectsProviderThumbnail(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_get_EffectsProviderSettingsLabel(This, value) \
    ((This)->lpVtbl->get_EffectsProviderSettingsLabel(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Not supported starting in windows 10")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_ShowSettingsUI(This) \
    ((This)->lpVtbl->ShowSettingsUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIAudioRenderEffectsManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IBasicAudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IBasicAudioEffect[] = L"Windows.Media.Effects.IBasicAudioEffect";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UseInputFrameForOutput)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedEncodingProperties)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        __FIVectorView_1_Windows__CMedia__CMediaProperties__CAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* SetEncodingProperties)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties);
    HRESULT (STDMETHODCALLTYPE* ProcessFrame)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* context);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason reason);
    HRESULT (STDMETHODCALLTYPE* DiscardQueuedFrames)(__x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffectVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_get_UseInputFrameForOutput(This, value) \
    ((This)->lpVtbl->get_UseInputFrameForOutput(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_get_SupportedEncodingProperties(This, value) \
    ((This)->lpVtbl->get_SupportedEncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_SetEncodingProperties(This, encodingProperties) \
    ((This)->lpVtbl->SetEncodingProperties(This, encodingProperties))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_ProcessFrame(This, context) \
    ((This)->lpVtbl->ProcessFrame(This, context))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_Close(This, reason) \
    ((This)->lpVtbl->Close(This, reason))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_DiscardQueuedFrames(This) \
    ((This)->lpVtbl->DiscardQueuedFrames(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicAudioEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IBasicVideoEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IBasicVideoEffect[] = L"Windows.Media.Effects.IBasicVideoEffect";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedMemoryTypes)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CMediaMemoryTypes* value);
    HRESULT (STDMETHODCALLTYPE* get_TimeIndependent)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedEncodingProperties)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        __FIVectorView_1_Windows__CMedia__CMediaProperties__CVideoEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* SetEncodingProperties)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device);
    HRESULT (STDMETHODCALLTYPE* ProcessFrame)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* context);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason reason);
    HRESULT (STDMETHODCALLTYPE* DiscardQueuedFrames)(__x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffectVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_get_SupportedMemoryTypes(This, value) \
    ((This)->lpVtbl->get_SupportedMemoryTypes(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_get_TimeIndependent(This, value) \
    ((This)->lpVtbl->get_TimeIndependent(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_get_SupportedEncodingProperties(This, value) \
    ((This)->lpVtbl->get_SupportedEncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_SetEncodingProperties(This, encodingProperties, device) \
    ((This)->lpVtbl->SetEncodingProperties(This, encodingProperties, device))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_ProcessFrame(This, context) \
    ((This)->lpVtbl->ProcessFrame(This, context))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_Close(This, reason) \
    ((This)->lpVtbl->Close(This, reason))

#define __x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_DiscardQueuedFrames(This) \
    ((This)->lpVtbl->DiscardQueuedFrames(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIBasicVideoEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.ICompositeVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.CompositeVideoFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_ICompositeVideoFrameContext[] = L"Windows.Media.Effects.ICompositeVideoFrameContext";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SurfacesToOverlay)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        __FIVectorView_1_Windows__CGraphics__CDirectX__CDirect3D11__CIDirect3DSurface** value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundFrame)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputFrame)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);
    HRESULT (STDMETHODCALLTYPE* GetOverlayForSurface)(__x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* surfaceToOverlay,
        __x_ABI_CWindows_CMedia_CEditing_CIMediaOverlay** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContextVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_get_SurfacesToOverlay(This, value) \
    ((This)->lpVtbl->get_SurfacesToOverlay(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_get_BackgroundFrame(This, value) \
    ((This)->lpVtbl->get_BackgroundFrame(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_get_OutputFrame(This, value) \
    ((This)->lpVtbl->get_OutputFrame(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_GetOverlayForSurface(This, surfaceToOverlay, value) \
    ((This)->lpVtbl->GetOverlayForSurface(This, surfaceToOverlay, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IProcessAudioFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.ProcessAudioFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IProcessAudioFrameContext[] = L"Windows.Media.Effects.IProcessAudioFrameContext";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputFrame)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputFrame)(__x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContextVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_get_InputFrame(This, value) \
    ((This)->lpVtbl->get_InputFrame(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_get_OutputFrame(This, value) \
    ((This)->lpVtbl->get_OutputFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessAudioFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IProcessVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.ProcessVideoFrameContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IProcessVideoFrameContext[] = L"Windows.Media.Effects.IProcessVideoFrameContext";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputFrame)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_OutputFrame)(__x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContextVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_get_InputFrame(This, value) \
    ((This)->lpVtbl->get_InputFrame(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_get_OutputFrame(This, value) \
    ((This)->lpVtbl->get_OutputFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIProcessVideoFrameContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtension
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositor[] = L"Windows.Media.Effects.IVideoCompositor";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TimeIndependent)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* SetEncodingProperties)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* backgroundProperties,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device);
    HRESULT (STDMETHODCALLTYPE* CompositeFrame)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        __x_ABI_CWindows_CMedia_CEffects_CICompositeVideoFrameContext* context);
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This,
        enum __x_ABI_CWindows_CMedia_CEffects_CMediaEffectClosedReason reason);
    HRESULT (STDMETHODCALLTYPE* DiscardQueuedFrames)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_get_TimeIndependent(This, value) \
    ((This)->lpVtbl->get_TimeIndependent(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_SetEncodingProperties(This, backgroundProperties, device) \
    ((This)->lpVtbl->SetEncodingProperties(This, backgroundProperties, device))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_CompositeFrame(This, context) \
    ((This)->lpVtbl->CompositeFrame(This, context))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_Close(This, reason) \
    ((This)->lpVtbl->Close(This, reason))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_DiscardQueuedFrames(This) \
    ((This)->lpVtbl->DiscardQueuedFrames(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositorDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositorDefinition[] = L"Windows.Media.Effects.IVideoCompositorDefinition";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivatableClassId)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_get_ActivatableClassId(This, value) \
    ((This)->lpVtbl->get_ActivatableClassId(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoCompositorDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoCompositorDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoCompositorDefinitionFactory[] = L"Windows.Media.Effects.IVideoCompositorDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithProperties)(__x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* props,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_Create(This, activatableClassId, value) \
    ((This)->lpVtbl->Create(This, activatableClassId, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_CreateWithProperties(This, activatableClassId, props, value) \
    ((This)->lpVtbl->CreateWithProperties(This, activatableClassId, props, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoCompositorDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoEffectDefinition[] = L"Windows.Media.Effects.IVideoEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivatableClassId)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_get_ActivatableClassId(This, value) \
    ((This)->lpVtbl->get_ActivatableClassId(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoEffectDefinitionFactory[] = L"Windows.Media.Effects.IVideoEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithProperties)(__x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory* This,
        HSTRING activatableClassId,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* props,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_Create(This, activatableClassId, value) \
    ((This)->lpVtbl->Create(This, activatableClassId, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_CreateWithProperties(This, activatableClassId, props, value) \
    ((This)->lpVtbl->CreateWithProperties(This, activatableClassId, props, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IVideoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformEffectDefinition[] = L"Windows.Media.Effects.IVideoTransformEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PaddingColor)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_PaddingColor)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_OutputSize)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* put_OutputSize)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* get_CropRectangle)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_CropRectangle)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* get_Rotation)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaRotation* value);
    HRESULT (STDMETHODCALLTYPE* put_Rotation)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaRotation value);
    HRESULT (STDMETHODCALLTYPE* get_Mirror)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaMirroringOptions* value);
    HRESULT (STDMETHODCALLTYPE* put_Mirror)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaMirroringOptions value);
    HRESULT (STDMETHODCALLTYPE* put_ProcessingAlgorithm)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm value);
    HRESULT (STDMETHODCALLTYPE* get_ProcessingAlgorithm)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CMediaVideoProcessingAlgorithm* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_PaddingColor(This, value) \
    ((This)->lpVtbl->get_PaddingColor(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_PaddingColor(This, value) \
    ((This)->lpVtbl->put_PaddingColor(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_OutputSize(This, value) \
    ((This)->lpVtbl->get_OutputSize(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_OutputSize(This, value) \
    ((This)->lpVtbl->put_OutputSize(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_CropRectangle(This, value) \
    ((This)->lpVtbl->get_CropRectangle(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_CropRectangle(This, value) \
    ((This)->lpVtbl->put_CropRectangle(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_Rotation(This, value) \
    ((This)->lpVtbl->get_Rotation(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_Rotation(This, value) \
    ((This)->lpVtbl->put_Rotation(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_Mirror(This, value) \
    ((This)->lpVtbl->get_Mirror(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_Mirror(This, value) \
    ((This)->lpVtbl->put_Mirror(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_put_ProcessingAlgorithm(This, value) \
    ((This)->lpVtbl->put_ProcessingAlgorithm(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_get_ProcessingAlgorithm(This, value) \
    ((This)->lpVtbl->get_ProcessingAlgorithm(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformEffectDefinition2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformEffectDefinition2[] = L"Windows.Media.Effects.IVideoTransformEffectDefinition2";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SphericalProjection)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2* This,
        __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2Vtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_get_SphericalProjection(This, value) \
    ((This)->lpVtbl->get_SphericalProjection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformEffectDefinition2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Effects.IVideoTransformSphericalProjection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Effects.VideoTransformSphericalProjection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Effects_IVideoTransformSphericalProjection[] = L"Windows.Media.Effects.IVideoTransformSphericalProjection";
typedef struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_FrameFormat)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat* value);
    HRESULT (STDMETHODCALLTYPE* put_FrameFormat)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat value);
    HRESULT (STDMETHODCALLTYPE* get_ProjectionMode)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        enum __x_ABI_CWindows_CMedia_CPlayback_CSphericalVideoProjectionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ProjectionMode)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        enum __x_ABI_CWindows_CMedia_CPlayback_CSphericalVideoProjectionMode value);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalFieldOfViewInDegrees)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_HorizontalFieldOfViewInDegrees)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_ViewOrientation)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* put_ViewOrientation)(__x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjectionVtbl;

interface __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_get_FrameFormat(This, value) \
    ((This)->lpVtbl->get_FrameFormat(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_put_FrameFormat(This, value) \
    ((This)->lpVtbl->put_FrameFormat(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_get_ProjectionMode(This, value) \
    ((This)->lpVtbl->get_ProjectionMode(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_put_ProjectionMode(This, value) \
    ((This)->lpVtbl->put_ProjectionMode(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_get_HorizontalFieldOfViewInDegrees(This, value) \
    ((This)->lpVtbl->get_HorizontalFieldOfViewInDegrees(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_put_HorizontalFieldOfViewInDegrees(This, value) \
    ((This)->lpVtbl->put_HorizontalFieldOfViewInDegrees(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_get_ViewOrientation(This, value) \
    ((This)->lpVtbl->get_ViewOrientation(This, value))

#define __x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_put_ViewOrientation(This, value) \
    ((This)->lpVtbl->put_ViewOrientation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CEffects_CIVideoTransformSphericalProjection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Effects.AcousticEchoCancellationConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 19.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAcousticEchoCancellationConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AcousticEchoCancellationConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AcousticEchoCancellationConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AcousticEchoCancellationConfiguration[] = L"Windows.Media.Effects.AcousticEchoCancellationConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x130000

/*
 *
 * Class Windows.Media.Effects.AudioCaptureEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioCaptureEffectsManager ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioCaptureEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioCaptureEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioCaptureEffectsManager[] = L"Windows.Media.Effects.AudioCaptureEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioEffect ** Default Interface **
 *    Windows.Media.Effects.IAudioEffect2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffect_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffect[] = L"Windows.Media.Effects.AudioEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IAudioEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioEffectDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffectDefinition[] = L"Windows.Media.Effects.AudioEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Effects.IAudioEffectsManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioEffectsManager[] = L"Windows.Media.Effects.AudioEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.AudioRenderEffectsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IAudioRenderEffectsManager ** Default Interface **
 *    Windows.Media.Effects.IAudioRenderEffectsManager2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_AudioRenderEffectsManager_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_AudioRenderEffectsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_AudioRenderEffectsManager[] = L"Windows.Media.Effects.AudioRenderEffectsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.CompositeVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.ICompositeVideoFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_CompositeVideoFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_CompositeVideoFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_CompositeVideoFrameContext[] = L"Windows.Media.Effects.CompositeVideoFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.ProcessAudioFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IProcessAudioFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_ProcessAudioFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_ProcessAudioFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_ProcessAudioFrameContext[] = L"Windows.Media.Effects.ProcessAudioFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.ProcessVideoFrameContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IProcessVideoFrameContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_ProcessVideoFrameContext_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_ProcessVideoFrameContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_ProcessVideoFrameContext[] = L"Windows.Media.Effects.ProcessVideoFrameContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoCompositorDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IVideoCompositorDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoCompositorDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoCompositorDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoCompositorDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoCompositorDefinition[] = L"Windows.Media.Effects.VideoCompositorDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Effects.IVideoEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoEffectDefinition ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoEffectDefinition[] = L"Windows.Media.Effects.VideoEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoTransformEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IVideoTransformEffectDefinition
 *    Windows.Media.Effects.IVideoTransformEffectDefinition2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoTransformEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoTransformEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoTransformEffectDefinition[] = L"Windows.Media.Effects.VideoTransformEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Effects.VideoTransformSphericalProjection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Effects.IVideoTransformSphericalProjection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Effects_VideoTransformSphericalProjection_DEFINED
#define RUNTIMECLASS_Windows_Media_Effects_VideoTransformSphericalProjection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Effects_VideoTransformSphericalProjection[] = L"Windows.Media.Effects.VideoTransformSphericalProjection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Eeffects_p_h__

#endif // __windows2Emedia2Eeffects_h__
