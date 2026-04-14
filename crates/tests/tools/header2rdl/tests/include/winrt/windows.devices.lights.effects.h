
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
#ifndef __windows2Edevices2Elights2Eeffects_h__
#define __windows2Edevices2Elights2Eeffects_h__
#ifndef __windows2Edevices2Elights2Eeffects_p_h__
#define __windows2Edevices2Elights2Eeffects_p_h__


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
#include "Windows.Devices.Lights.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayBitmapEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayBitmapEffectFactory;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapEffectFactory

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayBitmapRequestedEventArgs;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayBlinkEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect ABI::Windows::Devices::Lights::Effects::ILampArrayBlinkEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayBlinkEffectFactory;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory ABI::Windows::Devices::Lights::Effects::ILampArrayBlinkEffectFactory

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayColorRampEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect ABI::Windows::Devices::Lights::Effects::ILampArrayColorRampEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayColorRampEffectFactory;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory ABI::Windows::Devices::Lights::Effects::ILampArrayColorRampEffectFactory

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayCustomEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect ABI::Windows::Devices::Lights::Effects::ILampArrayCustomEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayCustomEffectFactory;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory ABI::Windows::Devices::Lights::Effects::ILampArrayCustomEffectFactory

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect ABI::Windows::Devices::Lights::Effects::ILampArrayEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayEffectPlaylist;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist ABI::Windows::Devices::Lights::Effects::ILampArrayEffectPlaylist

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayEffectPlaylistStatics;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics ABI::Windows::Devices::Lights::Effects::ILampArrayEffectPlaylistStatics

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArraySolidEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect ABI::Windows::Devices::Lights::Effects::ILampArraySolidEffect

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArraySolidEffectFactory;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory ABI::Windows::Devices::Lights::Effects::ILampArraySolidEffectFactory

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    interface ILampArrayUpdateRequestedEventArgs;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs ABI::Windows::Devices::Lights::Effects::ILampArrayUpdateRequestedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#define DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de51580c-48a6-50b5-976b-05894699015a"))
IIterator<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> : IIterator_impl<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Lights.Effects.ILampArrayEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t;
#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#define DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3f108d37-6679-5590-aed2-033362fbf413"))
IIterable<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> : IIterable_impl<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Lights.Effects.ILampArrayEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t;
#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayEffectPlaylist;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE
#define DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2aaabbc2-4c18-5d1c-9e09-c1249eb46817"))
IIterator<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*, ABI::Windows::Devices::Lights::Effects::ILampArrayEffectPlaylist*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Lights.Effects.LampArrayEffectPlaylist>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*> __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_t;
#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE
#define DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2314acda-c5df-5051-977d-94d79d1312fb"))
IIterable<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*, ABI::Windows::Devices::Lights::Effects::ILampArrayEffectPlaylist*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Lights.Effects.LampArrayEffectPlaylist>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Lights::Effects::LampArrayEffectPlaylist*> __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_t;
#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#define DEF___FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("97159586-9fb0-56d4-9df4-8c36ea15100e"))
IVectorView<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> : IVectorView_impl<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Lights.Effects.ILampArrayEffect>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Lights::Effects::ILampArrayEffect*> __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t;
#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayBitmapEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayBitmapRequestedEventArgs;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("24b5818b-448e-53fa-ab4c-663008c5d4cf"))
ITypedEventHandler<ABI::Windows::Devices::Lights::Effects::LampArrayBitmapEffect*, ABI::Windows::Devices::Lights::Effects::LampArrayBitmapRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayBitmapEffect*, ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapEffect*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayBitmapRequestedEventArgs*, ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Lights.Effects.LampArrayBitmapEffect, Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Lights::Effects::LampArrayBitmapEffect*, ABI::Windows::Devices::Lights::Effects::LampArrayBitmapRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayCustomEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayUpdateRequestedEventArgs;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7d91af6e-ba44-5a0f-bc64-3901fd33661c"))
ITypedEventHandler<ABI::Windows::Devices::Lights::Effects::LampArrayCustomEffect*, ABI::Windows::Devices::Lights::Effects::LampArrayUpdateRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayCustomEffect*, ABI::Windows::Devices::Lights::Effects::ILampArrayCustomEffect*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Lights::Effects::LampArrayUpdateRequestedEventArgs*, ABI::Windows::Devices::Lights::Effects::ILampArrayUpdateRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Lights.Effects.LampArrayCustomEffect, Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Lights::Effects::LampArrayCustomEffect*, ABI::Windows::Devices::Lights::Effects::LampArrayUpdateRequestedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                class LampArray;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                interface ILampArray;
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CLights_CILampArray ABI::Windows::Devices::Lights::ILampArray

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    typedef enum LampArrayEffectCompletionBehavior : int LampArrayEffectCompletionBehavior;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    typedef enum LampArrayEffectStartMode : int LampArrayEffectStartMode;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    typedef enum LampArrayRepetitionMode : int LampArrayRepetitionMode;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayBlinkEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArrayColorRampEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    class LampArraySolidEffect;
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    enum LampArrayEffectCompletionBehavior : int
                    {
                        LampArrayEffectCompletionBehavior_ClearState = 0,
                        LampArrayEffectCompletionBehavior_KeepState = 1,
                    };
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayEffectStartMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    enum LampArrayEffectStartMode : int
                    {
                        LampArrayEffectStartMode_Sequential = 0,
                        LampArrayEffectStartMode_Simultaneous = 1,
                    };
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayRepetitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    enum LampArrayRepetitionMode : int
                    {
                        LampArrayRepetitionMode_Occurrences = 0,
                        LampArrayRepetitionMode_Forever = 1,
                    };
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("3238e065-d877-4627-89e5-2a88f7052fa6")
                    ILampArrayBitmapEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Duration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartDelay(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StartDelay(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UpdateInterval(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UpdateInterval(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SuggestedBitmapSize(
                            ABI::Windows::Foundation::Size* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_BitmapRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_BitmapRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayBitmapEffect = __uuidof(ILampArrayBitmapEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("13608090-e336-4c8f-9053-a92407ca7b1d")
                    ILampArrayBitmapEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Lights::ILampArray* lampArray,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes,
                            ABI::Windows::Devices::Lights::Effects::ILampArrayBitmapEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayBitmapEffectFactory = __uuidof(ILampArrayBitmapEffectFactory);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("c8b4af9e-fe63-4d51-babd-619defb454ba")
                    ILampArrayBitmapRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SinceStarted(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateBitmap(
                            ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayBitmapRequestedEventArgs = __uuidof(ILampArrayBitmapRequestedEventArgs);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBlinkEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBlinkEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayBlinkEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("ebbf35f6-2fc5-4bb3-b3c3-6221a7680d13")
                    ILampArrayBlinkEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Color(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Color(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AttackDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AttackDuration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SustainDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_SustainDuration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DecayDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DecayDuration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RepetitionDelay(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RepetitionDelay(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartDelay(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StartDelay(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Occurrences(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Occurrences(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RepetitionMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayRepetitionMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RepetitionMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayRepetitionMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayBlinkEffect = __uuidof(ILampArrayBlinkEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBlinkEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("879f1d97-9f50-49b2-a56f-013aa08d55e0")
                    ILampArrayBlinkEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Lights::ILampArray* lampArray,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes,
                            ABI::Windows::Devices::Lights::Effects::ILampArrayBlinkEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayBlinkEffectFactory = __uuidof(ILampArrayBlinkEffectFactory);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayColorRampEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayColorRampEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayColorRampEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("2b004437-40a7-432e-a0b9-0d570c2153ff")
                    ILampArrayColorRampEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Color(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Color(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RampDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RampDuration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartDelay(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StartDelay(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompletionBehavior(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectCompletionBehavior* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CompletionBehavior(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectCompletionBehavior value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayColorRampEffect = __uuidof(ILampArrayColorRampEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayColorRampEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("520bd133-0c74-4df5-bea7-4899e0266b0f")
                    ILampArrayColorRampEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Lights::ILampArray* lampArray,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes,
                            ABI::Windows::Devices::Lights::Effects::ILampArrayColorRampEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayColorRampEffectFactory = __uuidof(ILampArrayColorRampEffectFactory);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayCustomEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayCustomEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayCustomEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("ec579170-3c34-4876-818b-5765f78b0ee4")
                    ILampArrayCustomEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Duration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UpdateInterval(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UpdateInterval(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UpdateRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UpdateRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayCustomEffect = __uuidof(ILampArrayCustomEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayCustomEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("68b4774d-63e5-4af0-a58b-3e535b94e8c9")
                    ILampArrayCustomEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Lights::ILampArray* lampArray,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes,
                            ABI::Windows::Devices::Lights::Effects::ILampArrayCustomEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayCustomEffectFactory = __uuidof(ILampArrayCustomEffectFactory);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("11d45590-57fb-4546-b1ce-863107f740df")
                    ILampArrayEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ZIndex(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ZIndex(
                            INT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayEffect = __uuidof(ILampArrayEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffectPlaylist[] = L"Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("7de58bfe-6f61-4103-98c7-d6632f7b9169")
                    ILampArrayEffectPlaylist : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Append(
                            ABI::Windows::Devices::Lights::Effects::ILampArrayEffect* effect
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OverrideZIndex(
                            INT32 zIndex
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EffectStartMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectStartMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_EffectStartMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectStartMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Occurrences(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Occurrences(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RepetitionMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayRepetitionMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RepetitionMode(
                            ABI::Windows::Devices::Lights::Effects::LampArrayRepetitionMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayEffectPlaylist = __uuidof(ILampArrayEffectPlaylist);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffectPlaylistStatics[] = L"Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("fb15235c-ea35-4c7f-a016-f3bfc6a6c47d")
                    ILampArrayEffectPlaylistStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE StartAll(
                            __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAll(
                            __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PauseAll(
                            __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayEffectPlaylistStatics = __uuidof(ILampArrayEffectPlaylistStatics);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArraySolidEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArraySolidEffect[] = L"Windows.Devices.Lights.Effects.ILampArraySolidEffect";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("441f8213-43cc-4b33-80eb-c6ddde7dc8ed")
                    ILampArraySolidEffect : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Color(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Color(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Duration(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartDelay(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StartDelay(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompletionBehavior(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectCompletionBehavior* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CompletionBehavior(
                            ABI::Windows::Devices::Lights::Effects::LampArrayEffectCompletionBehavior value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArraySolidEffect = __uuidof(ILampArraySolidEffect);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArraySolidEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("f862a32c-5576-4341-961b-aee1f13cf9dd")
                    ILampArraySolidEffectFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Devices::Lights::ILampArray* lampArray,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes,
                            ABI::Windows::Devices::Lights::Effects::ILampArraySolidEffect** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArraySolidEffectFactory = __uuidof(ILampArraySolidEffectFactory);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayUpdateRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Lights {
                namespace Effects {
                    MIDL_INTERFACE("73560d6a-576a-48af-8539-67ffa0ab3516")
                    ILampArrayUpdateRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SinceStarted(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetColor(
                            ABI::Windows::UI::Color desiredColor
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetColorForIndex(
                            INT32 lampIndex,
                            ABI::Windows::UI::Color desiredColor
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSingleColorForIndices(
                            ABI::Windows::UI::Color desiredColor,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetColorsForIndices(
                            UINT32 desiredColorsLength,
                            ABI::Windows::UI::Color* desiredColors,
                            UINT32 lampIndexesLength,
                            INT32* lampIndexes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ILampArrayUpdateRequestedEventArgs = __uuidof(ILampArrayUpdateRequestedEventArgs);
                } /* Effects */
            } /* Lights */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBitmapEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBitmapEffect[] = L"Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBlinkEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBlinkEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBlinkEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBlinkEffect[] = L"Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayColorRampEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayColorRampEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayColorRampEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayColorRampEffect[] = L"Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayCustomEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayCustomEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayCustomEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayCustomEffect[] = L"Windows.Devices.Lights.Effects.LampArrayCustomEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Lights.Effects.ILampArrayEffect>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Devices.Lights.Effects.ILampArrayEffect>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist[] = L"Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArraySolidEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArraySolidEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArraySolidEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArraySolidEffect[] = L"Windows.Devices.Lights.Effects.LampArraySolidEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

typedef struct __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl;

interface __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

typedef struct __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        __FIIterator_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl;

interface __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist;

typedef struct __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl;

interface __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist;

typedef struct __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* This,
        __FIIterator_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl;

interface __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylistVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect;

typedef struct __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl;

interface __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CLights__CEffects__CILampArrayEffect_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* sender,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* sender,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CLights_CILampArray __x_ABI_CWindows_CDevices_CLights_CILampArray;

#endif // ____x_ABI_CWindows_CDevices_CLights_CILampArray_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior;

typedef enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectStartMode __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectStartMode;

typedef enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode;

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior
{
    LampArrayEffectCompletionBehavior_ClearState = 0,
    LampArrayEffectCompletionBehavior_KeepState = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayEffectStartMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectStartMode
{
    LampArrayEffectStartMode_Sequential = 0,
    LampArrayEffectStartMode_Simultaneous = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Devices.Lights.Effects.LampArrayRepetitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode
{
    LampArrayRepetitionMode_Occurrences = 0,
    LampArrayRepetitionMode_Forever = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateInterval)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_UpdateInterval)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedBitmapSize)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* add_BitmapRequested)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayBitmapEffect_Windows__CDevices__CLights__CEffects__CLampArrayBitmapRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BitmapRequested)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_get_StartDelay(This, value) \
    ((This)->lpVtbl->get_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_put_StartDelay(This, value) \
    ((This)->lpVtbl->put_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_get_UpdateInterval(This, value) \
    ((This)->lpVtbl->get_UpdateInterval(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_put_UpdateInterval(This, value) \
    ((This)->lpVtbl->put_UpdateInterval(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_get_SuggestedBitmapSize(This, value) \
    ((This)->lpVtbl->get_SuggestedBitmapSize(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_add_BitmapRequested(This, handler, token) \
    ((This)->lpVtbl->add_BitmapRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_remove_BitmapRequested(This, token) \
    ((This)->lpVtbl->remove_BitmapRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* lampArray,
        UINT32 lampIndexesLength,
        INT32* lampIndexes,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value) \
    ((This)->lpVtbl->CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBitmapRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SinceStarted)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* UpdateBitmap)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_get_SinceStarted(This, value) \
    ((This)->lpVtbl->get_SinceStarted(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_UpdateBitmap(This, bitmap) \
    ((This)->lpVtbl->UpdateBitmap(This, bitmap))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBitmapRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBlinkEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBlinkEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayBlinkEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_AttackDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_AttackDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_SustainDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_SustainDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_DecayDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DecayDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_RepetitionDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_RepetitionDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_Occurrences)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Occurrences)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_RepetitionMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_RepetitionMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_AttackDuration(This, value) \
    ((This)->lpVtbl->get_AttackDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_AttackDuration(This, value) \
    ((This)->lpVtbl->put_AttackDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_SustainDuration(This, value) \
    ((This)->lpVtbl->get_SustainDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_SustainDuration(This, value) \
    ((This)->lpVtbl->put_SustainDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_DecayDuration(This, value) \
    ((This)->lpVtbl->get_DecayDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_DecayDuration(This, value) \
    ((This)->lpVtbl->put_DecayDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_RepetitionDelay(This, value) \
    ((This)->lpVtbl->get_RepetitionDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_RepetitionDelay(This, value) \
    ((This)->lpVtbl->put_RepetitionDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_StartDelay(This, value) \
    ((This)->lpVtbl->get_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_StartDelay(This, value) \
    ((This)->lpVtbl->put_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_Occurrences(This, value) \
    ((This)->lpVtbl->get_Occurrences(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_Occurrences(This, value) \
    ((This)->lpVtbl->put_Occurrences(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_get_RepetitionMode(This, value) \
    ((This)->lpVtbl->get_RepetitionMode(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_put_RepetitionMode(This, value) \
    ((This)->lpVtbl->put_RepetitionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayBlinkEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* lampArray,
        UINT32 lampIndexesLength,
        INT32* lampIndexes,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value) \
    ((This)->lpVtbl->CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayBlinkEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayColorRampEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayColorRampEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayColorRampEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_RampDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_RampDuration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_CompletionBehavior)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_CompletionBehavior)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_get_RampDuration(This, value) \
    ((This)->lpVtbl->get_RampDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_put_RampDuration(This, value) \
    ((This)->lpVtbl->put_RampDuration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_get_StartDelay(This, value) \
    ((This)->lpVtbl->get_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_put_StartDelay(This, value) \
    ((This)->lpVtbl->put_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_get_CompletionBehavior(This, value) \
    ((This)->lpVtbl->get_CompletionBehavior(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_put_CompletionBehavior(This, value) \
    ((This)->lpVtbl->put_CompletionBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayColorRampEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* lampArray,
        UINT32 lampIndexesLength,
        INT32* lampIndexes,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value) \
    ((This)->lpVtbl->CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayColorRampEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayCustomEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayCustomEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayCustomEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateInterval)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_UpdateInterval)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* add_UpdateRequested)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        __FITypedEventHandler_2_Windows__CDevices__CLights__CEffects__CLampArrayCustomEffect_Windows__CDevices__CLights__CEffects__CLampArrayUpdateRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UpdateRequested)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_get_UpdateInterval(This, value) \
    ((This)->lpVtbl->get_UpdateInterval(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_put_UpdateInterval(This, value) \
    ((This)->lpVtbl->put_UpdateInterval(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_add_UpdateRequested(This, handler, token) \
    ((This)->lpVtbl->add_UpdateRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_remove_UpdateRequested(This, token) \
    ((This)->lpVtbl->remove_UpdateRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayCustomEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* lampArray,
        UINT32 lampIndexesLength,
        INT32* lampIndexes,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value) \
    ((This)->lpVtbl->CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayCustomEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffect[] = L"Windows.Devices.Lights.Effects.ILampArrayEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ZIndex)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ZIndex)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_get_ZIndex(This, value) \
    ((This)->lpVtbl->get_ZIndex(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_put_ZIndex(This, value) \
    ((This)->lpVtbl->put_ZIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffectPlaylist[] = L"Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Append)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffect* effect);
    HRESULT (STDMETHODCALLTYPE* OverrideZIndex)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        INT32 zIndex);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This);
    HRESULT (STDMETHODCALLTYPE* get_EffectStartMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectStartMode* value);
    HRESULT (STDMETHODCALLTYPE* put_EffectStartMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectStartMode value);
    HRESULT (STDMETHODCALLTYPE* get_Occurrences)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Occurrences)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_RepetitionMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_RepetitionMode)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayRepetitionMode value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_Append(This, effect) \
    ((This)->lpVtbl->Append(This, effect))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_OverrideZIndex(This, zIndex) \
    ((This)->lpVtbl->OverrideZIndex(This, zIndex))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_get_EffectStartMode(This, value) \
    ((This)->lpVtbl->get_EffectStartMode(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_put_EffectStartMode(This, value) \
    ((This)->lpVtbl->put_EffectStartMode(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_get_Occurrences(This, value) \
    ((This)->lpVtbl->get_Occurrences(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_put_Occurrences(This, value) \
    ((This)->lpVtbl->put_Occurrences(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_get_RepetitionMode(This, value) \
    ((This)->lpVtbl->get_RepetitionMode(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_put_RepetitionMode(This, value) \
    ((This)->lpVtbl->put_RepetitionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylist_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayEffectPlaylistStatics[] = L"Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAll)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value);
    HRESULT (STDMETHODCALLTYPE* StopAll)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value);
    HRESULT (STDMETHODCALLTYPE* PauseAll)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics* This,
        __FIIterable_1_Windows__CDevices__CLights__CEffects__CLampArrayEffectPlaylist* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_StartAll(This, value) \
    ((This)->lpVtbl->StartAll(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_StopAll(This, value) \
    ((This)->lpVtbl->StopAll(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_PauseAll(This, value) \
    ((This)->lpVtbl->PauseAll(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayEffectPlaylistStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArraySolidEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArraySolidEffect[] = L"Windows.Devices.Lights.Effects.ILampArraySolidEffect";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_StartDelay)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_CompletionBehavior)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_CompletionBehavior)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect* This,
        enum __x_ABI_CWindows_CDevices_CLights_CEffects_CLampArrayEffectCompletionBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_get_StartDelay(This, value) \
    ((This)->lpVtbl->get_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_put_StartDelay(This, value) \
    ((This)->lpVtbl->put_StartDelay(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_get_CompletionBehavior(This, value) \
    ((This)->lpVtbl->get_CompletionBehavior(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_put_CompletionBehavior(This, value) \
    ((This)->lpVtbl->put_CompletionBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArraySolidEffectFactory[] = L"Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory* This,
        __x_ABI_CWindows_CDevices_CLights_CILampArray* lampArray,
        UINT32 lampIndexesLength,
        INT32* lampIndexes,
        __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffect** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value) \
    ((This)->lpVtbl->CreateInstance(This, lampArray, lampIndexesLength, lampIndexes, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArraySolidEffectFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Lights_Effects_ILampArrayUpdateRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SinceStarted)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* SetColor)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor);
    HRESULT (STDMETHODCALLTYPE* SetColorForIndex)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        INT32 lampIndex,
        struct __x_ABI_CWindows_CUI_CColor desiredColor);
    HRESULT (STDMETHODCALLTYPE* SetSingleColorForIndices)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        struct __x_ABI_CWindows_CUI_CColor desiredColor,
        UINT32 lampIndexesLength,
        INT32* lampIndexes);
    HRESULT (STDMETHODCALLTYPE* SetColorsForIndices)(__x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs* This,
        UINT32 desiredColorsLength,
        struct __x_ABI_CWindows_CUI_CColor* desiredColors,
        UINT32 lampIndexesLength,
        INT32* lampIndexes);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_get_SinceStarted(This, value) \
    ((This)->lpVtbl->get_SinceStarted(This, value))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_SetColor(This, desiredColor) \
    ((This)->lpVtbl->SetColor(This, desiredColor))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_SetColorForIndex(This, lampIndex, desiredColor) \
    ((This)->lpVtbl->SetColorForIndex(This, lampIndex, desiredColor))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_SetSingleColorForIndices(This, desiredColor, lampIndexesLength, lampIndexes) \
    ((This)->lpVtbl->SetSingleColorForIndices(This, desiredColor, lampIndexesLength, lampIndexes))

#define __x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_SetColorsForIndices(This, desiredColorsLength, desiredColors, lampIndexesLength, lampIndexes) \
    ((This)->lpVtbl->SetColorsForIndices(This, desiredColorsLength, desiredColors, lampIndexesLength, lampIndexes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CLights_CEffects_CILampArrayUpdateRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBitmapEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayBitmapEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBitmapEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBitmapEffect[] = L"Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBitmapRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBitmapRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayBlinkEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayBlinkEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayBlinkEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBlinkEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayBlinkEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayBlinkEffect[] = L"Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayColorRampEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayColorRampEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayColorRampEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayColorRampEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayColorRampEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayColorRampEffect[] = L"Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayCustomEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArrayCustomEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayCustomEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayCustomEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayCustomEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayCustomEffect[] = L"Windows.Devices.Lights.Effects.LampArrayCustomEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayEffectPlaylist
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Lights.Effects.ILampArrayEffectPlaylistStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayEffectPlaylist ** Default Interface **
 *    Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Lights.Effects.ILampArrayEffect>
 *    Windows.Foundation.Collections.IIterable`1<Windows.Devices.Lights.Effects.ILampArrayEffect>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayEffectPlaylist[] = L"Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArraySolidEffect
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Lights.Effects.ILampArraySolidEffectFactory interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArraySolidEffect ** Default Interface **
 *    Windows.Devices.Lights.Effects.ILampArrayEffect
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArraySolidEffect_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArraySolidEffect_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArraySolidEffect[] = L"Windows.Devices.Lights.Effects.LampArraySolidEffect";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Lights.Effects.ILampArrayUpdateRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Lights_Effects_LampArrayUpdateRequestedEventArgs[] = L"Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
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
#endif // __windows2Edevices2Elights2Eeffects_p_h__

#endif // __windows2Edevices2Elights2Eeffects_h__
