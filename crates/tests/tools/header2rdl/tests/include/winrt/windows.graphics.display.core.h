
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
#ifndef __windows2Egraphics2Edisplay2Ecore_h__
#define __windows2Egraphics2Edisplay2Ecore_h__
#ifndef __windows2Egraphics2Edisplay2Ecore_p_h__
#define __windows2Egraphics2Edisplay2Ecore_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    interface IHdmiDisplayInformation;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation ABI::Windows::Graphics::Display::Core::IHdmiDisplayInformation

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    interface IHdmiDisplayInformationStatics;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics ABI::Windows::Graphics::Display::Core::IHdmiDisplayInformationStatics

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    interface IHdmiDisplayMode;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    interface IHdmiDisplayMode2;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2 ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode2

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__

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
        namespace Graphics {
            namespace Display {
                namespace Core {
                    class HdmiDisplayMode;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#define DEF___FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d66eb831-e22c-5ee3-af45-e1c03de4bc62"))
IIterator<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*, ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Graphics.Display.Core.HdmiDisplayMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t;
#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#define DEF___FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("497e3d51-0ea1-5be0-8dba-8f7f4ce4fb33"))
IIterable<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*, ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Graphics.Display.Core.HdmiDisplayMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t;
#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#define DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7d0e7c64-df0e-539a-ab5f-3c260026c5ce"))
IVectorView<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*, ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Graphics.Display.Core.HdmiDisplayMode>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Graphics::Display::Core::HdmiDisplayMode*> __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t;
#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    class HdmiDisplayInformation;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d109932b-9ce1-5cdd-94c7-93c60c833aa3"))
ITypedEventHandler<ABI::Windows::Graphics::Display::Core::HdmiDisplayInformation*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Display::Core::HdmiDisplayInformation*, ABI::Windows::Graphics::Display::Core::IHdmiDisplayInformation*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Display.Core.HdmiDisplayInformation, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Display::Core::HdmiDisplayInformation*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    typedef enum HdmiDisplayColorSpace : int HdmiDisplayColorSpace;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    typedef enum HdmiDisplayHdrOption : int HdmiDisplayHdrOption;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    typedef enum HdmiDisplayPixelEncoding : int HdmiDisplayPixelEncoding;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    typedef struct HdmiDisplayHdr2086Metadata HdmiDisplayHdr2086Metadata;
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    enum HdmiDisplayColorSpace : int
                    {
                        HdmiDisplayColorSpace_RgbLimited = 0,
                        HdmiDisplayColorSpace_RgbFull = 1,
                        HdmiDisplayColorSpace_BT2020 = 2,
                        HdmiDisplayColorSpace_BT709 = 3,
                    };
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayHdrOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    enum HdmiDisplayHdrOption : int
                    {
                        HdmiDisplayHdrOption_None = 0,
                        HdmiDisplayHdrOption_EotfSdr = 1,
                        HdmiDisplayHdrOption_Eotf2084 = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        HdmiDisplayHdrOption_DolbyVisionLowLatency = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    };
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    enum HdmiDisplayPixelEncoding : int
                    {
                        HdmiDisplayPixelEncoding_Rgb444 = 0,
                        HdmiDisplayPixelEncoding_Ycc444 = 1,
                        HdmiDisplayPixelEncoding_Ycc422 = 2,
                        HdmiDisplayPixelEncoding_Ycc420 = 3,
                    };
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    struct HdmiDisplayHdr2086Metadata
                    {
                        UINT16 RedPrimaryX;
                        UINT16 RedPrimaryY;
                        UINT16 GreenPrimaryX;
                        UINT16 GreenPrimaryY;
                        UINT16 BluePrimaryX;
                        UINT16 BluePrimaryY;
                        UINT16 WhitePointX;
                        UINT16 WhitePointY;
                        UINT16 MaxMasteringLuminance;
                        UINT16 MinMasteringLuminance;
                        UINT16 MaxContentLightLevel;
                        UINT16 MaxFrameAverageLightLevel;
                    };
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayInformation[] = L"Windows.Graphics.Display.Core.IHdmiDisplayInformation";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("130b3c0a-f565-476e-abd5-ea05aee74c69")
                    IHdmiDisplayInformation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetSupportedDisplayModes(
                            __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCurrentDisplayMode(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDefaultDisplayModeAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestSetCurrentDisplayModeAsync(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode* mode,
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestSetCurrentDisplayModeWithHdrAsync(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode* mode,
                            ABI::Windows::Graphics::Display::Core::HdmiDisplayHdrOption hdrOption,
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode* mode,
                            ABI::Windows::Graphics::Display::Core::HdmiDisplayHdrOption hdrOption,
                            ABI::Windows::Graphics::Display::Core::HdmiDisplayHdr2086Metadata hdrMetadata,
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DisplayModesChanged(
                            __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* value,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DisplayModesChanged(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHdmiDisplayInformation = __uuidof(IHdmiDisplayInformation);
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayInformationStatics[] = L"Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("6ce6b260-f42a-4a15-914c-7b8e2a5a65df")
                    IHdmiDisplayInformationStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayInformation** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHdmiDisplayInformationStatics = __uuidof(IHdmiDisplayInformationStatics);
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayMode[] = L"Windows.Graphics.Display.Core.IHdmiDisplayMode";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("0c06d5ad-1b90-4f51-9981-ef5a1c0ddf66")
                    IHdmiDisplayMode : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResolutionWidthInRawPixels(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResolutionHeightInRawPixels(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RefreshRate(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StereoEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BitsPerPixel(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsEqual(
                            ABI::Windows::Graphics::Display::Core::IHdmiDisplayMode* mode,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColorSpace(
                            ABI::Windows::Graphics::Display::Core::HdmiDisplayColorSpace* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PixelEncoding(
                            ABI::Windows::Graphics::Display::Core::HdmiDisplayPixelEncoding* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSdrLuminanceSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSmpte2084Supported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Is2086MetadataSupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHdmiDisplayMode = __uuidof(IHdmiDisplayMode);
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayMode2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayMode2[] = L"Windows.Graphics.Display.Core.IHdmiDisplayMode2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Display {
                namespace Core {
                    MIDL_INTERFACE("07cd4e9f-4b3c-42b8-84e7-895368718af2")
                    IHdmiDisplayMode2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsDolbyVisionLowLatencySupported(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHdmiDisplayMode2 = __uuidof(IHdmiDisplayMode2);
                } /* Core */
            } /* Display */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.Core.IHdmiDisplayInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_Core_HdmiDisplayInformation[] = L"Windows.Graphics.Display.Core.HdmiDisplayInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.Core.IHdmiDisplayMode ** Default Interface **
 *    Windows.Graphics.Display.Core.IHdmiDisplayMode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayMode_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayMode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_Core_HdmiDisplayMode[] = L"Windows.Graphics.Display.Core.HdmiDisplayMode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2 __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2;

#endif // ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

typedef struct __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl;

interface __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode
{
    CONST_VTBL struct __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

typedef struct __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        __FIIterator_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl;

interface __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode
{
    CONST_VTBL struct __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode;

typedef struct __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        UINT32 index,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl;

interface __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayColorSpace __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayColorSpace;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdrOption __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdrOption;

typedef enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayPixelEncoding __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayPixelEncoding;

typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdr2086Metadata __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdr2086Metadata;

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayColorSpace
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayColorSpace
{
    HdmiDisplayColorSpace_RgbLimited = 0,
    HdmiDisplayColorSpace_RgbFull = 1,
    HdmiDisplayColorSpace_BT2020 = 2,
    HdmiDisplayColorSpace_BT709 = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayHdrOption
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdrOption
{
    HdmiDisplayHdrOption_None = 0,
    HdmiDisplayHdrOption_EotfSdr = 1,
    HdmiDisplayHdrOption_Eotf2084 = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    HdmiDisplayHdrOption_DolbyVisionLowLatency = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayPixelEncoding
{
    HdmiDisplayPixelEncoding_Rgb444 = 0,
    HdmiDisplayPixelEncoding_Ycc444 = 1,
    HdmiDisplayPixelEncoding_Ycc422 = 2,
    HdmiDisplayPixelEncoding_Ycc420 = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdr2086Metadata
{
    UINT16 RedPrimaryX;
    UINT16 RedPrimaryY;
    UINT16 GreenPrimaryX;
    UINT16 GreenPrimaryY;
    UINT16 BluePrimaryX;
    UINT16 BluePrimaryY;
    UINT16 WhitePointX;
    UINT16 WhitePointY;
    UINT16 MaxMasteringLuminance;
    UINT16 MinMasteringLuminance;
    UINT16 MaxContentLightLevel;
    UINT16 MaxFrameAverageLightLevel;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayInformation[] = L"Windows.Graphics.Display.Core.IHdmiDisplayInformation";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetSupportedDisplayModes)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __FIVectorView_1_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayMode** result);
    HRESULT (STDMETHODCALLTYPE* GetCurrentDisplayMode)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode** result);
    HRESULT (STDMETHODCALLTYPE* SetDefaultDisplayModeAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RequestSetCurrentDisplayModeAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* mode,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestSetCurrentDisplayModeWithHdrAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* mode,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdrOption hdrOption,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* mode,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdrOption hdrOption,
        struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayHdr2086Metadata hdrMetadata,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* add_DisplayModesChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        __FITypedEventHandler_2_Windows__CGraphics__CDisplay__CCore__CHdmiDisplayInformation_IInspectable* value,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisplayModesChanged)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_GetSupportedDisplayModes(This, result) \
    ((This)->lpVtbl->GetSupportedDisplayModes(This, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_GetCurrentDisplayMode(This, result) \
    ((This)->lpVtbl->GetCurrentDisplayMode(This, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_SetDefaultDisplayModeAsync(This, operation) \
    ((This)->lpVtbl->SetDefaultDisplayModeAsync(This, operation))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_RequestSetCurrentDisplayModeAsync(This, mode, operation) \
    ((This)->lpVtbl->RequestSetCurrentDisplayModeAsync(This, mode, operation))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_RequestSetCurrentDisplayModeWithHdrAsync(This, mode, hdrOption, operation) \
    ((This)->lpVtbl->RequestSetCurrentDisplayModeWithHdrAsync(This, mode, hdrOption, operation))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(This, mode, hdrOption, hdrMetadata, operation) \
    ((This)->lpVtbl->RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(This, mode, hdrOption, hdrMetadata, operation))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_add_DisplayModesChanged(This, value, token) \
    ((This)->lpVtbl->add_DisplayModesChanged(This, value, token))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_remove_DisplayModesChanged(This, token) \
    ((This)->lpVtbl->remove_DisplayModesChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayInformationStatics[] = L"Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformation** result);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStaticsVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_GetForCurrentView(This, result) \
    ((This)->lpVtbl->GetForCurrentView(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayInformationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayMode[] = L"Windows.Graphics.Display.Core.IHdmiDisplayMode";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayModeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResolutionWidthInRawPixels)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ResolutionHeightInRawPixels)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_RefreshRate)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_StereoEnabled)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_BitsPerPixel)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* IsEqual)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* mode,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_ColorSpace)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayColorSpace* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelEncoding)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        enum __x_ABI_CWindows_CGraphics_CDisplay_CCore_CHdmiDisplayPixelEncoding* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSdrLuminanceSupported)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSmpte2084Supported)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Is2086MetadataSupported)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayModeVtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayModeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_ResolutionWidthInRawPixels(This, value) \
    ((This)->lpVtbl->get_ResolutionWidthInRawPixels(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_ResolutionHeightInRawPixels(This, value) \
    ((This)->lpVtbl->get_ResolutionHeightInRawPixels(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_RefreshRate(This, value) \
    ((This)->lpVtbl->get_RefreshRate(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_StereoEnabled(This, value) \
    ((This)->lpVtbl->get_StereoEnabled(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_BitsPerPixel(This, value) \
    ((This)->lpVtbl->get_BitsPerPixel(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_IsEqual(This, mode, result) \
    ((This)->lpVtbl->IsEqual(This, mode, result))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_ColorSpace(This, value) \
    ((This)->lpVtbl->get_ColorSpace(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_PixelEncoding(This, value) \
    ((This)->lpVtbl->get_PixelEncoding(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_IsSdrLuminanceSupported(This, value) \
    ((This)->lpVtbl->get_IsSdrLuminanceSupported(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_IsSmpte2084Supported(This, value) \
    ((This)->lpVtbl->get_IsSmpte2084Supported(This, value))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_get_Is2086MetadataSupported(This, value) \
    ((This)->lpVtbl->get_Is2086MetadataSupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Graphics.Display.Core.IHdmiDisplayMode2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Display_Core_IHdmiDisplayMode2[] = L"Windows.Graphics.Display.Core.IHdmiDisplayMode2";
typedef struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDolbyVisionLowLatencySupported)(__x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2Vtbl;

interface __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_get_IsDolbyVisionLowLatencySupported(This, value) \
    ((This)->lpVtbl->get_IsDolbyVisionLowLatencySupported(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CDisplay_CCore_CIHdmiDisplayMode2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Graphics.Display.Core.HdmiDisplayInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Display.Core.IHdmiDisplayInformationStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.Core.IHdmiDisplayInformation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayInformation_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_Core_HdmiDisplayInformation[] = L"Windows.Graphics.Display.Core.HdmiDisplayInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Graphics.Display.Core.HdmiDisplayMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Display.Core.IHdmiDisplayMode ** Default Interface **
 *    Windows.Graphics.Display.Core.IHdmiDisplayMode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayMode_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Display_Core_HdmiDisplayMode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Display_Core_HdmiDisplayMode[] = L"Windows.Graphics.Display.Core.HdmiDisplayMode";
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
#endif // __windows2Egraphics2Edisplay2Ecore_p_h__

#endif // __windows2Egraphics2Edisplay2Ecore_h__
