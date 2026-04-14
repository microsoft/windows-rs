
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
#ifndef __windows2Egraphics2Edisplay_h__
#define __windows2Egraphics2Edisplay_h__
#ifndef __windows2Egraphics2Edisplay_p_h__
#define __windows2Egraphics2Edisplay_p_h__


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
#include "Windows.Graphics.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayPropertiesEventHandler;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IAdvancedColorInfo;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo ABI::Windows::Graphics::Display::IAdvancedColorInfo

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IBrightnessOverride;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride ABI::Windows::Graphics::Display::IBrightnessOverride

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IBrightnessOverrideSettings;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings ABI::Windows::Graphics::Display::IBrightnessOverrideSettings

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IBrightnessOverrideSettingsStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics ABI::Windows::Graphics::Display::IBrightnessOverrideSettingsStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IBrightnessOverrideStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics ABI::Windows::Graphics::Display::IBrightnessOverrideStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IColorOverrideSettings;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings ABI::Windows::Graphics::Display::IColorOverrideSettings

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IColorOverrideSettingsStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics ABI::Windows::Graphics::Display::IColorOverrideSettingsStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayEnhancementOverride;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride ABI::Windows::Graphics::Display::IDisplayEnhancementOverride

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayEnhancementOverrideCapabilities;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideCapabilities

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayEnhancementOverrideCapabilitiesChangedEventArgs;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideCapabilitiesChangedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayEnhancementOverrideStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformation;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation ABI::Windows::Graphics::Display::IDisplayInformation

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformation2;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2 ABI::Windows::Graphics::Display::IDisplayInformation2

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformation3;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3 ABI::Windows::Graphics::Display::IDisplayInformation3

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformation4;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4 ABI::Windows::Graphics::Display::IDisplayInformation4

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformation5;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5 ABI::Windows::Graphics::Display::IDisplayInformation5

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayInformationStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics ABI::Windows::Graphics::Display::IDisplayInformationStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayPropertiesStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics ABI::Windows::Graphics::Display::IDisplayPropertiesStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayServices;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices ABI::Windows::Graphics::Display::IDisplayServices

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                interface IDisplayServicesStatics;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics ABI::Windows::Graphics::Display::IDisplayServicesStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__

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


#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream ABI::Windows::Storage::Streams::IRandomAccessStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("430ecece-1418-5d19-81b2-5ddb381603cc"))
IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperation_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("398c4183-793d-5b00-819b-4aef92485e94"))
IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Storage::Streams::IRandomAccessStream*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Storage.Streams.IRandomAccessStream>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Storage::Streams::IRandomAccessStream*> __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef struct NitRange NitRange;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_USE
#define DEF___FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6be92993-d069-5a99-b9e8-200cf5c8a060"))
IIterator<struct ABI::Windows::Graphics::Display::NitRange> : IIterator_impl<struct ABI::Windows::Graphics::Display::NitRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Display.NitRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Graphics::Display::NitRange> __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_t;
#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_USE
#define DEF___FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7fb7a783-ce2d-552d-bee3-bc1442db0409"))
IIterable<struct ABI::Windows::Graphics::Display::NitRange> : IIterable_impl<struct ABI::Windows::Graphics::Display::NitRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Display.NitRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Graphics::Display::NitRange> __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_t;
#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cc8ef226-50c1-5efa-98c4-1043d0bf5b35"))
IVectorView<struct ABI::Windows::Graphics::Display::NitRange> : IVectorView_impl<struct ABI::Windows::Graphics::Display::NitRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Display.NitRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Graphics::Display::NitRange> __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_t;
#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000


#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */


namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class BrightnessOverride;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a460214e-6620-521d-9cb9-a0a0f732ce90"))
ITypedEventHandler<ABI::Windows::Graphics::Display::BrightnessOverride*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::BrightnessOverride*, ABI::Windows::Graphics::Display::IBrightnessOverride*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Display.BrightnessOverride, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Display::BrightnessOverride*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class DisplayEnhancementOverride;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3247b54b-7f00-5555-81df-afae022f0796"))
ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, ABI::Windows::Graphics::Display::IDisplayEnhancementOverride*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Display.DisplayEnhancementOverride, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class DisplayEnhancementOverrideCapabilitiesChangedEventArgs;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7e61af14-3e29-5039-92ee-3f2472b99e43"))
ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, ABI::Windows::Graphics::Display::DisplayEnhancementOverrideCapabilitiesChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, ABI::Windows::Graphics::Display::IDisplayEnhancementOverride*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::DisplayEnhancementOverrideCapabilitiesChangedEventArgs*, ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideCapabilitiesChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Display.DisplayEnhancementOverride, Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayEnhancementOverride*, ABI::Windows::Graphics::Display::DisplayEnhancementOverrideCapabilitiesChangedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class DisplayInformation;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("86c4f619-67b6-51c7-b30d-d8cf13625327"))
ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayInformation*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::DisplayInformation*, ABI::Windows::Graphics::Display::IDisplayInformation*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Display.DisplayInformation, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Display::DisplayInformation*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            typedef struct DisplayId DisplayId;
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum AdvancedColorKind : int AdvancedColorKind;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum DisplayBrightnessOverrideOptions : unsigned int DisplayBrightnessOverrideOptions;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum DisplayBrightnessOverrideScenario : int DisplayBrightnessOverrideScenario;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum DisplayBrightnessScenario : int DisplayBrightnessScenario;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum DisplayColorOverrideScenario : int DisplayColorOverrideScenario;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum DisplayOrientations : unsigned int DisplayOrientations;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum HdrMetadataFormat : int HdrMetadataFormat;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                typedef enum ResolutionScale : int ResolutionScale;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class AdvancedColorInfo;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class BrightnessOverrideSettings;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class ColorOverrideSettings;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                class DisplayEnhancementOverrideCapabilities;
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Display.AdvancedColorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum AdvancedColorKind : int
                {
                    AdvancedColorKind_StandardDynamicRange = 0,
                    AdvancedColorKind_WideColorGamut = 1,
                    AdvancedColorKind_HighDynamicRange = 2,
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessOverrideOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum DisplayBrightnessOverrideOptions : unsigned int
                {
                    DisplayBrightnessOverrideOptions_None = 0,
                    DisplayBrightnessOverrideOptions_UseDimmedPolicyWhenBatteryIsLow = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(DisplayBrightnessOverrideOptions)
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessOverrideScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum DisplayBrightnessOverrideScenario : int
                {
                    DisplayBrightnessOverrideScenario_IdleBrightness = 0,
                    DisplayBrightnessOverrideScenario_BarcodeReadingBrightness = 1,
                    DisplayBrightnessOverrideScenario_FullBrightness = 2,
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum DisplayBrightnessScenario : int
                {
                    DisplayBrightnessScenario_DefaultBrightness = 0,
                    DisplayBrightnessScenario_IdleBrightness = 1,
                    DisplayBrightnessScenario_BarcodeReadingBrightness = 2,
                    DisplayBrightnessScenario_FullBrightness = 3,
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.DisplayColorOverrideScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum DisplayColorOverrideScenario : int
                {
                    DisplayColorOverrideScenario_Accurate = 0,
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayOrientations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum DisplayOrientations : unsigned int
                {
                    DisplayOrientations_None = 0,
                    DisplayOrientations_Landscape = 0x1,
                    DisplayOrientations_Portrait = 0x2,
                    DisplayOrientations_LandscapeFlipped = 0x4,
                    DisplayOrientations_PortraitFlipped = 0x8,
                };

                DEFINE_ENUM_FLAG_OPERATORS(DisplayOrientations)
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Display.HdrMetadataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum HdrMetadataFormat : int
                {
                    HdrMetadataFormat_Hdr10 = 0,
                    HdrMetadataFormat_Hdr10Plus = 1,
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.ResolutionScale
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                enum ResolutionScale : int
                {
                    ResolutionScale_Invalid = 0,
                    ResolutionScale_Scale100Percent = 100,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale120Percent = 120,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale125Percent = 125,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale140Percent = 140,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale150Percent = 150,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale160Percent = 160,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale175Percent = 175,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale180Percent = 180,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale200Percent = 200,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale225Percent = 225,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale250Percent = 250,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale300Percent = 300,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale350Percent = 350,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale400Percent = 400,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale450Percent = 450,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ResolutionScale_Scale500Percent = 500,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Display.NitRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                struct NitRange
                {
                    FLOAT MinNits;
                    FLOAT MaxNits;
                    FLOAT StepSizeNits;
                };
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Delegate Windows.Graphics.Display.DisplayPropertiesEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("dbdd8b01-f1a1-46d1-9ee3-543bcc995980")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IDisplayPropertiesEventHandler : public IUnknown
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        IInspectable* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayPropertiesEventHandler = __uuidof(IDisplayPropertiesEventHandler);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IAdvancedColorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.AdvancedColorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IAdvancedColorInfo[] = L"Windows.Graphics.Display.IAdvancedColorInfo";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("8797dcfb-b229-4081-ae9a-2cc85e34ad6a")
                IAdvancedColorInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentAdvancedColorKind(
                        ABI::Windows::Graphics::Display::AdvancedColorKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RedPrimary(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GreenPrimary(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BluePrimary(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WhitePoint(
                        ABI::Windows::Foundation::Point* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxLuminanceInNits(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinLuminanceInNits(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAverageFullFrameLuminanceInNits(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SdrWhiteLevelInNits(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsHdrMetadataFormatCurrentlySupported(
                        ABI::Windows::Graphics::Display::HdrMetadataFormat format,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsAdvancedColorKindAvailable(
                        ABI::Windows::Graphics::Display::AdvancedColorKind kind,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdvancedColorInfo = __uuidof(IAdvancedColorInfo);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverride[] = L"Windows.Graphics.Display.IBrightnessOverride";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("96c9621a-c143-4392-bedd-4a7e9574c8fd")
                IBrightnessOverride : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOverrideActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BrightnessLevel(
                        DOUBLE* level
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetBrightnessLevel(
                        DOUBLE brightnessLevel,
                        ABI::Windows::Graphics::Display::DisplayBrightnessOverrideOptions options
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetBrightnessScenario(
                        ABI::Windows::Graphics::Display::DisplayBrightnessScenario scenario,
                        ABI::Windows::Graphics::Display::DisplayBrightnessOverrideOptions options
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetLevelForScenario(
                        ABI::Windows::Graphics::Display::DisplayBrightnessScenario scenario,
                        DOUBLE* brightnessLevel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartOverride(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopOverride(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsSupportedChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsSupportedChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsOverrideActiveChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsOverrideActiveChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_BrightnessLevelChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_BrightnessLevelChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBrightnessOverride = __uuidof(IBrightnessOverride);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideSettings[] = L"Windows.Graphics.Display.IBrightnessOverrideSettings";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("d112ab2a-7604-4dba-bcf8-4b6f49502cb0")
                IBrightnessOverrideSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredLevel(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredNits(
                        FLOAT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBrightnessOverrideSettings = __uuidof(IBrightnessOverrideSettings);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideSettingsStatics[] = L"Windows.Graphics.Display.IBrightnessOverrideSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("d487dc90-6f74-440b-b383-5fe96cf00b0f")
                IBrightnessOverrideSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromLevel(
                        DOUBLE level,
                        ABI::Windows::Graphics::Display::IBrightnessOverrideSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromNits(
                        FLOAT nits,
                        ABI::Windows::Graphics::Display::IBrightnessOverrideSettings** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromDisplayBrightnessOverrideScenario(
                        ABI::Windows::Graphics::Display::DisplayBrightnessOverrideScenario overrideScenario,
                        ABI::Windows::Graphics::Display::IBrightnessOverrideSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBrightnessOverrideSettingsStatics = __uuidof(IBrightnessOverrideSettingsStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideStatics[] = L"Windows.Graphics.Display.IBrightnessOverrideStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("03a7b9ed-e1f1-4a68-a11f-946ad8ce5393")
                IBrightnessOverrideStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultForSystem(
                        ABI::Windows::Graphics::Display::IBrightnessOverride** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Graphics::Display::IBrightnessOverride** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveForSystemAsync(
                        ABI::Windows::Graphics::Display::IBrightnessOverride* value,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBrightnessOverrideStatics = __uuidof(IBrightnessOverrideStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.IColorOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.ColorOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IColorOverrideSettings[] = L"Windows.Graphics.Display.IColorOverrideSettings";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("fbefa134-4a81-4c4d-a5b6-7d1b5c4bd00b")
                IColorOverrideSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredDisplayColorOverrideScenario(
                        ABI::Windows::Graphics::Display::DisplayColorOverrideScenario* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IColorOverrideSettings = __uuidof(IColorOverrideSettings);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IColorOverrideSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.ColorOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IColorOverrideSettingsStatics[] = L"Windows.Graphics.Display.IColorOverrideSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("b068e05f-c41f-4ac9-afab-827ab6248f9a")
                IColorOverrideSettingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFromDisplayColorOverrideScenario(
                        ABI::Windows::Graphics::Display::DisplayColorOverrideScenario overrideScenario,
                        ABI::Windows::Graphics::Display::IColorOverrideSettings** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IColorOverrideSettingsStatics = __uuidof(IColorOverrideSettingsStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverride[] = L"Windows.Graphics.Display.IDisplayEnhancementOverride";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("429594cf-d97a-4b02-a428-5c4292f7f522")
                IDisplayEnhancementOverride : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ColorOverrideSettings(
                        ABI::Windows::Graphics::Display::IColorOverrideSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ColorOverrideSettings(
                        ABI::Windows::Graphics::Display::IColorOverrideSettings* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BrightnessOverrideSettings(
                        ABI::Windows::Graphics::Display::IBrightnessOverrideSettings** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BrightnessOverrideSettings(
                        ABI::Windows::Graphics::Display::IBrightnessOverrideSettings* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanOverride(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOverrideActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentDisplayEnhancementOverrideCapabilities(
                        ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideCapabilities** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestOverride(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopOverride(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_CanOverrideChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CanOverrideChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsOverrideActiveChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsOverrideActiveChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DisplayEnhancementOverrideCapabilitiesChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DisplayEnhancementOverrideCapabilitiesChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayEnhancementOverride = __uuidof(IDisplayEnhancementOverride);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideCapabilities[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("457060de-ee5a-47b7-9918-1e51e812ccc8")
                IDisplayEnhancementOverrideCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsBrightnessControlSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsBrightnessNitsControlSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSupportedNitRanges(
                        __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayEnhancementOverrideCapabilities = __uuidof(IDisplayEnhancementOverrideCapabilities);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("db61e664-15fa-49da-8b77-07dbd2af585d")
                IDisplayEnhancementOverrideCapabilitiesChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Capabilities(
                        ABI::Windows::Graphics::Display::IDisplayEnhancementOverrideCapabilities** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs = __uuidof(IDisplayEnhancementOverrideCapabilitiesChangedEventArgs);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideStatics[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("cf5b7ec1-9791-4453-b013-29b6f778e519")
                IDisplayEnhancementOverrideStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Graphics::Display::IDisplayEnhancementOverride** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayEnhancementOverrideStatics = __uuidof(IDisplayEnhancementOverrideStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation[] = L"Windows.Graphics.Display.IDisplayInformation";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("bed112ae-adc3-4dc9-ae65-851f4d7d4799")
                IDisplayInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentOrientation(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NativeOrientation(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_OrientationChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_OrientationChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResolutionScale(
                        ABI::Windows::Graphics::Display::ResolutionScale* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LogicalDpi(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawDpiX(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawDpiY(
                        FLOAT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DpiChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DpiChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StereoEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StereoEnabledChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StereoEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetColorProfileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ColorProfileChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ColorProfileChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformation = __uuidof(IDisplayInformation);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Display.IDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation2[] = L"Windows.Graphics.Display.IDisplayInformation2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("4dcd0021-fad1-4b8e-8edf-775887b8bf19")
                IDisplayInformation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RawPixelsPerViewPixel(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformation2 = __uuidof(IDisplayInformation2);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation3[] = L"Windows.Graphics.Display.IDisplayInformation3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("db15011d-0f09-4466-8ff3-11de9a3c929a")
                IDisplayInformation3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DiagonalSizeInInches(
                        __FIReference_1_double** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformation3 = __uuidof(IDisplayInformation3);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation4[] = L"Windows.Graphics.Display.IDisplayInformation4";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("c972ce2f-1242-46be-b536-e1aafe9e7acf")
                IDisplayInformation4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ScreenWidthInRawPixels(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScreenHeightInRawPixels(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformation4 = __uuidof(IDisplayInformation4);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation5[] = L"Windows.Graphics.Display.IDisplayInformation5";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("3a5442dc-2cde-4a8d-80d1-21dc5adcc1aa")
                IDisplayInformation5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetAdvancedColorInfo(
                        ABI::Windows::Graphics::Display::IAdvancedColorInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AdvancedColorInfoChanged(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AdvancedColorInfoChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformation5 = __uuidof(IDisplayInformation5);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformationStatics[] = L"Windows.Graphics.Display.IDisplayInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("c6a02a6c-d452-44dc-ba07-96f3c6adf9d1")
                IDisplayInformationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                        ABI::Windows::Graphics::Display::IDisplayInformation** current
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AutoRotationPreferences(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AutoRotationPreferences(
                        ABI::Windows::Graphics::Display::DisplayOrientations value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DisplayContentsInvalidated(
                        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DisplayContentsInvalidated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayInformationStatics = __uuidof(IDisplayInformationStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayPropertiesStatics[] = L"Windows.Graphics.Display.IDisplayPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("6937ed8d-30ea-4ded-8271-4553ff02f68a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                IDisplayPropertiesStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentOrientation(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_NativeOrientation(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_AutoRotationPreferences(
                        ABI::Windows::Graphics::Display::DisplayOrientations* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_AutoRotationPreferences(
                        ABI::Windows::Graphics::Display::DisplayOrientations value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_OrientationChanged(
                        ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_OrientationChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_ResolutionScale(
                        ABI::Windows::Graphics::Display::ResolutionScale* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_LogicalDpi(
                        FLOAT* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_LogicalDpiChanged(
                        ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_LogicalDpiChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_StereoEnabled(
                        boolean* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_StereoEnabledChanged(
                        ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_StereoEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetColorProfileAsync(
                        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_ColorProfileChanged(
                        ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_ColorProfileChanged(
                        EventRegistrationToken token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_DisplayContentsInvalidated(
                        ABI::Windows::Graphics::Display::IDisplayPropertiesEventHandler* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_DisplayContentsInvalidated(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayPropertiesStatics = __uuidof(IDisplayPropertiesStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayServices[] = L"Windows.Graphics.Display.IDisplayServices";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("1b54f32b-890d-5747-bd26-fdbdeb0c8a71")
                IDisplayServices : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IDisplayServices = __uuidof(IDisplayServices);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayServicesStatics[] = L"Windows.Graphics.Display.IDisplayServicesStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                MIDL_INTERFACE("dc2096bf-730a-5560-b461-91c13d692e0c")
                IDisplayServicesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FindAll(
                        UINT32* resultLength,
                        ABI::Windows::Graphics::DisplayId** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDisplayServicesStatics = __uuidof(IDisplayServicesStatics);
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Graphics.Display.AdvancedColorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IAdvancedColorInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_AdvancedColorInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_AdvancedColorInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_AdvancedColorInfo[] = L"Windows.Graphics.Display.AdvancedColorInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.BrightnessOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IBrightnessOverrideStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IBrightnessOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverride_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_BrightnessOverride[] = L"Windows.Graphics.Display.BrightnessOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Display.BrightnessOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IBrightnessOverrideSettingsStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IBrightnessOverrideSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverrideSettings_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverrideSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_BrightnessOverrideSettings[] = L"Windows.Graphics.Display.BrightnessOverrideSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.ColorOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IColorOverrideSettingsStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IColorOverrideSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_ColorOverrideSettings_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_ColorOverrideSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_ColorOverrideSettings[] = L"Windows.Graphics.Display.ColorOverrideSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayEnhancementOverrideStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverride_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverride[] = L"Windows.Graphics.Display.DisplayEnhancementOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities[] = L"Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayInformation ** Default Interface **
 *    Windows.Graphics.Display.IDisplayInformation2
 *    Windows.Graphics.Display.IDisplayInformation3
 *    Windows.Graphics.Display.IDisplayInformation4
 *    Windows.Graphics.Display.IDisplayInformation5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayInformation[] = L"Windows.Graphics.Display.DisplayInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Display.DisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayProperties_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayProperties[] = L"Windows.Graphics.Display.DisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Display.DisplayServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayServicesStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayServices ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayServices_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayServices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayServices[] = L"Windows.Graphics.Display.DisplayServices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2 __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3 __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4 __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5 __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStreamVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CStorage__CStreams__CIRandomAccessStream_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange __x_ABI_CWindows_CGraphics_CDisplay_CNitRange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CDisplay__CNitRange;

typedef struct __FIIterator_1_Windows__CGraphics__CDisplay__CNitRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CDisplay__CNitRange* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CDisplay__CNitRangeVtbl;

interface __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CDisplay__CNitRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CDisplay__CNitRange;

typedef struct __FIIterable_1_Windows__CGraphics__CDisplay__CNitRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CDisplay__CNitRange* This,
        __FIIterator_1_Windows__CGraphics__CDisplay__CNitRange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CDisplay__CNitRangeVtbl;

interface __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CDisplay__CNitRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange;

typedef struct __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRangeVtbl;

interface __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* sender,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

typedef struct __x_ABI_CWindows_CGraphics_CDisplayId __x_ABI_CWindows_CGraphics_CDisplayId;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CAdvancedColorKind __x_ABI_CWindows_CGraphics_CDisplay_CAdvancedColorKind;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideOptions __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideOptions;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideScenario __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideScenario;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessScenario __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessScenario;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayColorOverrideScenario __x_ABI_CWindows_CGraphics_CDisplay_CDisplayColorOverrideScenario;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CHdrMetadataFormat __x_ABI_CWindows_CGraphics_CDisplay_CHdrMetadataFormat;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CResolutionScale __x_ABI_CWindows_CGraphics_CDisplay_CResolutionScale;

/*
 *
 * Struct Windows.Graphics.Display.AdvancedColorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CDisplay_CAdvancedColorKind
{
    AdvancedColorKind_StandardDynamicRange = 0,
    AdvancedColorKind_WideColorGamut = 1,
    AdvancedColorKind_HighDynamicRange = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessOverrideOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideOptions
{
    DisplayBrightnessOverrideOptions_None = 0,
    DisplayBrightnessOverrideOptions_UseDimmedPolicyWhenBatteryIsLow = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessOverrideScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideScenario
{
    DisplayBrightnessOverrideScenario_IdleBrightness = 0,
    DisplayBrightnessOverrideScenario_BarcodeReadingBrightness = 1,
    DisplayBrightnessOverrideScenario_FullBrightness = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayBrightnessScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessScenario
{
    DisplayBrightnessScenario_DefaultBrightness = 0,
    DisplayBrightnessScenario_IdleBrightness = 1,
    DisplayBrightnessScenario_BarcodeReadingBrightness = 2,
    DisplayBrightnessScenario_FullBrightness = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.DisplayColorOverrideScenario
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayColorOverrideScenario
{
    DisplayColorOverrideScenario_Accurate = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.DisplayOrientations
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations
{
    DisplayOrientations_None = 0,
    DisplayOrientations_Landscape = 0x1,
    DisplayOrientations_Portrait = 0x2,
    DisplayOrientations_LandscapeFlipped = 0x4,
    DisplayOrientations_PortraitFlipped = 0x8,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Display.HdrMetadataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CGraphics_CDisplay_CHdrMetadataFormat
{
    HdrMetadataFormat_Hdr10 = 0,
    HdrMetadataFormat_Hdr10Plus = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Graphics.Display.ResolutionScale
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CDisplay_CResolutionScale
{
    ResolutionScale_Invalid = 0,
    ResolutionScale_Scale100Percent = 100,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale120Percent = 120,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale125Percent = 125,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale140Percent = 140,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale150Percent = 150,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale160Percent = 160,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale175Percent = 175,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale180Percent = 180,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale200Percent = 200,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale225Percent = 225,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale250Percent = 250,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale300Percent = 300,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale350Percent = 350,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale400Percent = 400,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale450Percent = 450,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ResolutionScale_Scale500Percent = 500,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Display.NitRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
struct __x_ABI_CWindows_CGraphics_CDisplay_CNitRange
{
    FLOAT MinNits;
    FLOAT MaxNits;
    FLOAT StepSizeNits;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Delegate Windows.Graphics.Display.DisplayPropertiesEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* This,
        IInspectable* sender);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandlerVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IAdvancedColorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.AdvancedColorInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IAdvancedColorInfo[] = L"Windows.Graphics.Display.IAdvancedColorInfo";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentAdvancedColorKind)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CAdvancedColorKind* value);
    HRESULT (STDMETHODCALLTYPE* get_RedPrimary)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_GreenPrimary)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_BluePrimary)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_WhitePoint)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxLuminanceInNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_MinLuminanceInNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxAverageFullFrameLuminanceInNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_SdrWhiteLevelInNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* IsHdrMetadataFormatCurrentlySupported)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CHdrMetadataFormat format,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsAdvancedColorKindAvailable)(__x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CAdvancedColorKind kind,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfoVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_CurrentAdvancedColorKind(This, value) \
    ((This)->lpVtbl->get_CurrentAdvancedColorKind(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_RedPrimary(This, value) \
    ((This)->lpVtbl->get_RedPrimary(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_GreenPrimary(This, value) \
    ((This)->lpVtbl->get_GreenPrimary(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_BluePrimary(This, value) \
    ((This)->lpVtbl->get_BluePrimary(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_WhitePoint(This, value) \
    ((This)->lpVtbl->get_WhitePoint(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_MaxLuminanceInNits(This, value) \
    ((This)->lpVtbl->get_MaxLuminanceInNits(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_MinLuminanceInNits(This, value) \
    ((This)->lpVtbl->get_MinLuminanceInNits(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_MaxAverageFullFrameLuminanceInNits(This, value) \
    ((This)->lpVtbl->get_MaxAverageFullFrameLuminanceInNits(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_get_SdrWhiteLevelInNits(This, value) \
    ((This)->lpVtbl->get_SdrWhiteLevelInNits(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_IsHdrMetadataFormatCurrentlySupported(This, format, result) \
    ((This)->lpVtbl->IsHdrMetadataFormatCurrentlySupported(This, format, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_IsAdvancedColorKindAvailable(This, kind, result) \
    ((This)->lpVtbl->IsAdvancedColorKindAvailable(This, kind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverride[] = L"Windows.Graphics.Display.IBrightnessOverride";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSupported)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOverrideActive)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_BrightnessLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        DOUBLE* level);
    HRESULT (STDMETHODCALLTYPE* SetBrightnessLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        DOUBLE brightnessLevel,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideOptions options);
    HRESULT (STDMETHODCALLTYPE* SetBrightnessScenario)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessScenario scenario,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideOptions options);
    HRESULT (STDMETHODCALLTYPE* GetLevelForScenario)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessScenario scenario,
        DOUBLE* brightnessLevel);
    HRESULT (STDMETHODCALLTYPE* StartOverride)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This);
    HRESULT (STDMETHODCALLTYPE* StopOverride)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This);
    HRESULT (STDMETHODCALLTYPE* add_IsSupportedChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsSupportedChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_IsOverrideActiveChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsOverrideActiveChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_BrightnessLevelChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CBrightnessOverride_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_BrightnessLevelChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_get_IsSupported(This, value) \
    ((This)->lpVtbl->get_IsSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_get_IsOverrideActive(This, value) \
    ((This)->lpVtbl->get_IsOverrideActive(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_get_BrightnessLevel(This, level) \
    ((This)->lpVtbl->get_BrightnessLevel(This, level))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_SetBrightnessLevel(This, brightnessLevel, options) \
    ((This)->lpVtbl->SetBrightnessLevel(This, brightnessLevel, options))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_SetBrightnessScenario(This, scenario, options) \
    ((This)->lpVtbl->SetBrightnessScenario(This, scenario, options))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_GetLevelForScenario(This, scenario, brightnessLevel) \
    ((This)->lpVtbl->GetLevelForScenario(This, scenario, brightnessLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_StartOverride(This) \
    ((This)->lpVtbl->StartOverride(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_StopOverride(This) \
    ((This)->lpVtbl->StopOverride(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_add_IsSupportedChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsSupportedChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_remove_IsSupportedChanged(This, token) \
    ((This)->lpVtbl->remove_IsSupportedChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_add_IsOverrideActiveChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsOverrideActiveChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_remove_IsOverrideActiveChanged(This, token) \
    ((This)->lpVtbl->remove_IsOverrideActiveChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_add_BrightnessLevelChanged(This, handler, token) \
    ((This)->lpVtbl->add_BrightnessLevelChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_remove_BrightnessLevelChanged(This, token) \
    ((This)->lpVtbl->remove_BrightnessLevelChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideSettings[] = L"Windows.Graphics.Display.IBrightnessOverrideSettings";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_get_DesiredLevel(This, value) \
    ((This)->lpVtbl->get_DesiredLevel(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_get_DesiredNits(This, value) \
    ((This)->lpVtbl->get_DesiredNits(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideSettingsStatics[] = L"Windows.Graphics.Display.IBrightnessOverrideSettingsStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        DOUBLE level,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromNits)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        FLOAT nits,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromDisplayBrightnessOverrideScenario)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayBrightnessOverrideScenario overrideScenario,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_CreateFromLevel(This, level, result) \
    ((This)->lpVtbl->CreateFromLevel(This, level, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_CreateFromNits(This, nits, result) \
    ((This)->lpVtbl->CreateFromNits(This, nits, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_CreateFromDisplayBrightnessOverrideScenario(This, overrideScenario, result) \
    ((This)->lpVtbl->CreateFromDisplayBrightnessOverrideScenario(This, overrideScenario, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IBrightnessOverrideStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.BrightnessOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IBrightnessOverrideStatics[] = L"Windows.Graphics.Display.IBrightnessOverrideStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefaultForSystem)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride** value);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride** value);
    HRESULT (STDMETHODCALLTYPE* SaveForSystemAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverride* value,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_GetDefaultForSystem(This, value) \
    ((This)->lpVtbl->GetDefaultForSystem(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_SaveForSystemAsync(This, value, operation) \
    ((This)->lpVtbl->SaveForSystemAsync(This, value, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.IColorOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.ColorOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IColorOverrideSettings[] = L"Windows.Graphics.Display.IColorOverrideSettings";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredDisplayColorOverrideScenario)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayColorOverrideScenario* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_get_DesiredDisplayColorOverrideScenario(This, value) \
    ((This)->lpVtbl->get_DesiredDisplayColorOverrideScenario(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IColorOverrideSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.ColorOverrideSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IColorOverrideSettingsStatics[] = L"Windows.Graphics.Display.IColorOverrideSettingsStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFromDisplayColorOverrideScenario)(__x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayColorOverrideScenario overrideScenario,
        __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_CreateFromDisplayColorOverrideScenario(This, overrideScenario, result) \
    ((This)->lpVtbl->CreateFromDisplayColorOverrideScenario(This, overrideScenario, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverride[] = L"Windows.Graphics.Display.IDisplayEnhancementOverride";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ColorOverrideSettings)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings** value);
    HRESULT (STDMETHODCALLTYPE* put_ColorOverrideSettings)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIColorOverrideSettings* value);
    HRESULT (STDMETHODCALLTYPE* get_BrightnessOverrideSettings)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings** value);
    HRESULT (STDMETHODCALLTYPE* put_BrightnessOverrideSettings)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIBrightnessOverrideSettings* value);
    HRESULT (STDMETHODCALLTYPE* get_CanOverride)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOverrideActive)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentDisplayEnhancementOverrideCapabilities)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* RequestOverride)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This);
    HRESULT (STDMETHODCALLTYPE* StopOverride)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This);
    HRESULT (STDMETHODCALLTYPE* add_CanOverrideChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CanOverrideChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_IsOverrideActiveChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsOverrideActiveChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DisplayEnhancementOverrideCapabilitiesChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayEnhancementOverride_Windows__CGraphics__CDisplay__CDisplayEnhancementOverrideCapabilitiesChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisplayEnhancementOverrideCapabilitiesChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_get_ColorOverrideSettings(This, value) \
    ((This)->lpVtbl->get_ColorOverrideSettings(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_put_ColorOverrideSettings(This, value) \
    ((This)->lpVtbl->put_ColorOverrideSettings(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_get_BrightnessOverrideSettings(This, value) \
    ((This)->lpVtbl->get_BrightnessOverrideSettings(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_put_BrightnessOverrideSettings(This, value) \
    ((This)->lpVtbl->put_BrightnessOverrideSettings(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_get_CanOverride(This, value) \
    ((This)->lpVtbl->get_CanOverride(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_get_IsOverrideActive(This, value) \
    ((This)->lpVtbl->get_IsOverrideActive(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_GetCurrentDisplayEnhancementOverrideCapabilities(This, value) \
    ((This)->lpVtbl->GetCurrentDisplayEnhancementOverrideCapabilities(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_RequestOverride(This) \
    ((This)->lpVtbl->RequestOverride(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_StopOverride(This) \
    ((This)->lpVtbl->StopOverride(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_add_CanOverrideChanged(This, handler, token) \
    ((This)->lpVtbl->add_CanOverrideChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_remove_CanOverrideChanged(This, token) \
    ((This)->lpVtbl->remove_CanOverrideChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_add_IsOverrideActiveChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsOverrideActiveChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_remove_IsOverrideActiveChanged(This, token) \
    ((This)->lpVtbl->remove_IsOverrideActiveChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_add_DisplayEnhancementOverrideCapabilitiesChanged(This, handler, token) \
    ((This)->lpVtbl->add_DisplayEnhancementOverrideCapabilitiesChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_remove_DisplayEnhancementOverrideCapabilitiesChanged(This, token) \
    ((This)->lpVtbl->remove_DisplayEnhancementOverrideCapabilitiesChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideCapabilities[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsBrightnessControlSupported)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBrightnessNitsControlSupported)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetSupportedNitRanges)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities* This,
        __FIVectorView_1_Windows__CGraphics__CDisplay__CNitRange** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_get_IsBrightnessControlSupported(This, value) \
    ((This)->lpVtbl->get_IsBrightnessControlSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_get_IsBrightnessNitsControlSupported(This, value) \
    ((This)->lpVtbl->get_IsBrightnessNitsControlSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_GetSupportedNitRanges(This, result) \
    ((This)->lpVtbl->GetSupportedNitRanges(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capabilities)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilities** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_get_Capabilities(This, value) \
    ((This)->lpVtbl->get_Capabilities(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideCapabilitiesChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayEnhancementOverrideStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayEnhancementOverride
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayEnhancementOverrideStatics[] = L"Windows.Graphics.Display.IDisplayEnhancementOverrideStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverride** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayEnhancementOverrideStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation[] = L"Windows.Graphics.Display.IDisplayInformation";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentOrientation)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
    HRESULT (STDMETHODCALLTYPE* get_NativeOrientation)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
    HRESULT (STDMETHODCALLTYPE* add_OrientationChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_OrientationChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_ResolutionScale)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CResolutionScale* value);
    HRESULT (STDMETHODCALLTYPE* get_LogicalDpi)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_RawDpiX)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_RawDpiY)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* add_DpiChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DpiChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_StereoEnabled)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_StereoEnabledChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StereoEnabledChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* GetColorProfileAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* add_ColorProfileChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ColorProfileChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_CurrentOrientation(This, value) \
    ((This)->lpVtbl->get_CurrentOrientation(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_NativeOrientation(This, value) \
    ((This)->lpVtbl->get_NativeOrientation(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_add_OrientationChanged(This, handler, token) \
    ((This)->lpVtbl->add_OrientationChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_remove_OrientationChanged(This, token) \
    ((This)->lpVtbl->remove_OrientationChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_ResolutionScale(This, value) \
    ((This)->lpVtbl->get_ResolutionScale(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_LogicalDpi(This, value) \
    ((This)->lpVtbl->get_LogicalDpi(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_RawDpiX(This, value) \
    ((This)->lpVtbl->get_RawDpiX(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_RawDpiY(This, value) \
    ((This)->lpVtbl->get_RawDpiY(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_add_DpiChanged(This, handler, token) \
    ((This)->lpVtbl->add_DpiChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_remove_DpiChanged(This, token) \
    ((This)->lpVtbl->remove_DpiChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_get_StereoEnabled(This, value) \
    ((This)->lpVtbl->get_StereoEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_add_StereoEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_StereoEnabledChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_remove_StereoEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_StereoEnabledChanged(This, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_GetColorProfileAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetColorProfileAsync(This, asyncInfo))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_add_ColorProfileChanged(This, handler, token) \
    ((This)->lpVtbl->add_ColorProfileChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_remove_ColorProfileChanged(This, token) \
    ((This)->lpVtbl->remove_ColorProfileChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Display.IDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation2[] = L"Windows.Graphics.Display.IDisplayInformation2";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RawPixelsPerViewPixel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2Vtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_get_RawPixelsPerViewPixel(This, value) \
    ((This)->lpVtbl->get_RawPixelsPerViewPixel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation3[] = L"Windows.Graphics.Display.IDisplayInformation3";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DiagonalSizeInInches)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3* This,
        __FIReference_1_double** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3Vtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_get_DiagonalSizeInInches(This, value) \
    ((This)->lpVtbl->get_DiagonalSizeInInches(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation4[] = L"Windows.Graphics.Display.IDisplayInformation4";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ScreenWidthInRawPixels)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ScreenHeightInRawPixels)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4Vtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_get_ScreenWidthInRawPixels(This, value) \
    ((This)->lpVtbl->get_ScreenWidthInRawPixels(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_get_ScreenHeightInRawPixels(This, value) \
    ((This)->lpVtbl->get_ScreenHeightInRawPixels(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformation5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformation5[] = L"Windows.Graphics.Display.IDisplayInformation5";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAdvancedColorInfo)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIAdvancedColorInfo** value);
    HRESULT (STDMETHODCALLTYPE* add_AdvancedColorInfoChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AdvancedColorInfoChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5Vtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_GetAdvancedColorInfo(This, value) \
    ((This)->lpVtbl->GetAdvancedColorInfo(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_add_AdvancedColorInfoChanged(This, handler, token) \
    ((This)->lpVtbl->add_AdvancedColorInfoChanged(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_remove_AdvancedColorInfoChanged(This, token) \
    ((This)->lpVtbl->remove_AdvancedColorInfoChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayInformationStatics[] = L"Windows.Graphics.Display.IDisplayInformationStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformation** current);
    HRESULT (STDMETHODCALLTYPE* get_AutoRotationPreferences)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoRotationPreferences)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations value);
    HRESULT (STDMETHODCALLTYPE* add_DisplayContentsInvalidated)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CDisplayInformation_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisplayContentsInvalidated)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_GetForCurrentView(This, current) \
    ((This)->lpVtbl->GetForCurrentView(This, current))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_get_AutoRotationPreferences(This, value) \
    ((This)->lpVtbl->get_AutoRotationPreferences(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_put_AutoRotationPreferences(This, value) \
    ((This)->lpVtbl->put_AutoRotationPreferences(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_add_DisplayContentsInvalidated(This, handler, token) \
    ((This)->lpVtbl->add_DisplayContentsInvalidated(This, handler, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_remove_DisplayContentsInvalidated(This, token) \
    ((This)->lpVtbl->remove_DisplayContentsInvalidated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayPropertiesStatics[] = L"Windows.Graphics.Display.IDisplayPropertiesStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_CurrentOrientation)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_NativeOrientation)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_AutoRotationPreferences)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_AutoRotationPreferences)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CDisplayOrientations value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_OrientationChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_OrientationChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ResolutionScale)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CResolutionScale* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_LogicalDpi)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        FLOAT* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_LogicalDpiChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_LogicalDpiChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_StereoEnabled)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        boolean* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_StereoEnabledChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_StereoEnabledChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetColorProfileAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __FIAsyncOperation_1_Windows__CStorage__CStreams__CIRandomAccessStream** asyncInfo);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_ColorProfileChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_ColorProfileChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_DisplayContentsInvalidated)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesEventHandler* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_DisplayContentsInvalidated)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_CurrentOrientation(This, value) \
    ((This)->lpVtbl->get_CurrentOrientation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_NativeOrientation(This, value) \
    ((This)->lpVtbl->get_NativeOrientation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_AutoRotationPreferences(This, value) \
    ((This)->lpVtbl->get_AutoRotationPreferences(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_put_AutoRotationPreferences(This, value) \
    ((This)->lpVtbl->put_AutoRotationPreferences(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_add_OrientationChanged(This, handler, token) \
    ((This)->lpVtbl->add_OrientationChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_remove_OrientationChanged(This, token) \
    ((This)->lpVtbl->remove_OrientationChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_ResolutionScale(This, value) \
    ((This)->lpVtbl->get_ResolutionScale(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_LogicalDpi(This, value) \
    ((This)->lpVtbl->get_LogicalDpi(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_add_LogicalDpiChanged(This, handler, token) \
    ((This)->lpVtbl->add_LogicalDpiChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_remove_LogicalDpiChanged(This, token) \
    ((This)->lpVtbl->remove_LogicalDpiChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_get_StereoEnabled(This, value) \
    ((This)->lpVtbl->get_StereoEnabled(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_add_StereoEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_StereoEnabledChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_remove_StereoEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_StereoEnabledChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_GetColorProfileAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetColorProfileAsync(This, asyncInfo))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_add_ColorProfileChanged(This, handler, token) \
    ((This)->lpVtbl->add_ColorProfileChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_remove_ColorProfileChanged(This, token) \
    ((This)->lpVtbl->remove_ColorProfileChanged(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_add_DisplayContentsInvalidated(This, handler, token) \
    ((This)->lpVtbl->add_DisplayContentsInvalidated(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_remove_DisplayContentsInvalidated(This, token) \
    ((This)->lpVtbl->remove_DisplayContentsInvalidated(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayServices[] = L"Windows.Graphics.Display.IDisplayServices";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServices_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.Graphics.Display.IDisplayServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.DisplayServices
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_IDisplayServicesStatics[] = L"Windows.Graphics.Display.IDisplayServicesStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAll)(__x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics* This,
        UINT32* resultLength,
        struct __x_ABI_CWindows_CGraphics_CDisplayId** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_FindAll(This, resultLength, result) \
    ((This)->lpVtbl->FindAll(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CIDisplayServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Graphics.Display.AdvancedColorInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IAdvancedColorInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_AdvancedColorInfo_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_AdvancedColorInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_AdvancedColorInfo[] = L"Windows.Graphics.Display.AdvancedColorInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.BrightnessOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IBrightnessOverrideStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IBrightnessOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverride_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_BrightnessOverride[] = L"Windows.Graphics.Display.BrightnessOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Display.BrightnessOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IBrightnessOverrideSettingsStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IBrightnessOverrideSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverrideSettings_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_BrightnessOverrideSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_BrightnessOverrideSettings[] = L"Windows.Graphics.Display.BrightnessOverrideSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.ColorOverrideSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IColorOverrideSettingsStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IColorOverrideSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_ColorOverrideSettings_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_ColorOverrideSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_ColorOverrideSettings[] = L"Windows.Graphics.Display.ColorOverrideSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverride
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayEnhancementOverrideStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverride ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverride_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverride_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverride[] = L"Windows.Graphics.Display.DisplayEnhancementOverride";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilities[] = L"Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayEnhancementOverrideCapabilitiesChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayEnhancementOverrideCapabilitiesChangedEventArgs[] = L"Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Display.DisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayInformationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayInformation ** Default Interface **
 *    Windows.Graphics.Display.IDisplayInformation2
 *    Windows.Graphics.Display.IDisplayInformation3
 *    Windows.Graphics.Display.IDisplayInformation4
 *    Windows.Graphics.Display.IDisplayInformation5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayInformation[] = L"Windows.Graphics.Display.DisplayInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Display.DisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayProperties_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DisplayProperties may be altered or unavailable for releases after Windows Phone 8.1. Instead, use DisplayInformation.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayProperties[] = L"Windows.Graphics.Display.DisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Display.DisplayServices
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.IDisplayServicesStatics interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.IDisplayServices ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_DisplayServices_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_DisplayServices_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_DisplayServices[] = L"Windows.Graphics.Display.DisplayServices";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egraphics2Edisplay_p_h__

#endif // __windows2Egraphics2Edisplay_h__
