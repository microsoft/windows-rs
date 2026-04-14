
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
#ifndef __windows2Esystem2Epreview_h__
#define __windows2Esystem2Epreview_h__
#ifndef __windows2Esystem2Epreview_p_h__
#define __windows2Esystem2Epreview_p_h__


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
#include "Windows.Devices.Sensors.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                interface ITwoPanelHingedDevicePosturePreview;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreview

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                interface ITwoPanelHingedDevicePosturePreviewReading;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReading

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                interface ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                interface ITwoPanelHingedDevicePosturePreviewStatics;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewStatics

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                class TwoPanelHingedDevicePosturePreview;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b9620919-e111-520f-8097-8e9b275391b5"))
IAsyncOperation<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.Preview.TwoPanelHingedDevicePosturePreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*> __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_t;
#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f2f2c54e-df52-5a7f-8610-e579fd8593d8"))
IAsyncOperationCompletedHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreview*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.Preview.TwoPanelHingedDevicePosturePreview>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                class TwoPanelHingedDevicePosturePreviewReading;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE
#define DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2286a430-a059-5998-810c-25ba16017c9e"))
IAsyncOperation<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReading*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*> __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_t;
#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4b1218ad-d399-5762-8202-c5d0d4b6eb54"))
IAsyncOperationCompletedHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReading*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReading*> __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                class TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3fc92c27-176d-58aa-a748-be82c378df41"))
ITypedEventHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*, ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreview*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs*, ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.Preview.TwoPanelHingedDevicePosturePreview, Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreview*, ABI::Windows::System::Preview::TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sensors {
                typedef enum SimpleOrientation : int SimpleOrientation;
            } /* Sensors */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                typedef enum HingeState : int HingeState;
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.Preview.HingeState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("HingeState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                HingeState : int
                {
                    HingeState_Unknown = 0,
                    HingeState_Closed = 1,
                    HingeState_Concave = 2,
                    HingeState_Flat = 3,
                    HingeState_Convex = 4,
                    HingeState_Full = 5,
                };
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreview[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreview";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                MIDL_INTERFACE("72245c31-4b39-42a6-8e73-7235ade16853")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ITwoPanelHingedDevicePosturePreview : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentPostureAsync(
                        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading** value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE add_PostureChanged(
                        __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE remove_PostureChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITwoPanelHingedDevicePosturePreview = __uuidof(ITwoPanelHingedDevicePosturePreview);
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewReading[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                MIDL_INTERFACE("a0251452-4ad6-4b38-8426-c59a15493a7d")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ITwoPanelHingedDevicePosturePreviewReading : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_HingeState(
                        ABI::Windows::System::Preview::HingeState* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Panel1Orientation(
                        ABI::Windows::Devices::Sensors::SimpleOrientation* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Panel1Id(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Panel2Orientation(
                        ABI::Windows::Devices::Sensors::SimpleOrientation* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Panel2Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITwoPanelHingedDevicePosturePreviewReading = __uuidof(ITwoPanelHingedDevicePosturePreviewReading);
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                MIDL_INTERFACE("2d2d1bc6-02ce-474a-a556-a75b1cf93a03")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE get_Reading(
                        ABI::Windows::System::Preview::ITwoPanelHingedDevicePosturePreviewReading** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs = __uuidof(ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs);
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewStatics[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace Preview {
                MIDL_INTERFACE("0c4733d2-57e0-4180-bd5e-f31a2138423e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                ITwoPanelHingedDevicePosturePreviewStatics : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITwoPanelHingedDevicePosturePreviewStatics = __uuidof(ITwoPanelHingedDevicePosturePreviewStatics);
            } /* Preview */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreview_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreview_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreview[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview;

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading;

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics;

#endif // ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* This,
        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading;

typedef struct __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl;

interface __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* This,
        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* sender,
        __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef enum __x_ABI_CWindows_CDevices_CSensors_CSimpleOrientation __x_ABI_CWindows_CDevices_CSensors_CSimpleOrientation;

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef enum __x_ABI_CWindows_CSystem_CPreview_CHingeState __x_ABI_CWindows_CSystem_CPreview_CHingeState;

/*
 *
 * Struct Windows.System.Preview.HingeState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("HingeState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSystem_CPreview_CHingeState
{
    HingeState_Unknown = 0,
    HingeState_Closed = 1,
    HingeState_Concave = 2,
    HingeState_Flat = 3,
    HingeState_Convex = 4,
    HingeState_Full = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreview[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreview";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetCurrentPostureAsync)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReading** value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* add_PostureChanged)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        __FITypedEventHandler_2_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* remove_PostureChanged)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewVtbl;

interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_GetCurrentPostureAsync(This, value) \
    ((This)->lpVtbl->GetCurrentPostureAsync(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_add_PostureChanged(This, handler, token) \
    ((This)->lpVtbl->add_PostureChanged(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_remove_PostureChanged(This, token) \
    ((This)->lpVtbl->remove_PostureChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreview_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewReading[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_HingeState)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        enum __x_ABI_CWindows_CSystem_CPreview_CHingeState* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Panel1Orientation)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        enum __x_ABI_CWindows_CDevices_CSensors_CSimpleOrientation* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Panel1Id)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Panel2Orientation)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        enum __x_ABI_CWindows_CDevices_CSensors_CSimpleOrientation* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Panel2Id)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingVtbl;

interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_HingeState(This, value) \
    ((This)->lpVtbl->get_HingeState(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_Panel1Orientation(This, value) \
    ((This)->lpVtbl->get_Panel1Orientation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_Panel1Id(This, value) \
    ((This)->lpVtbl->get_Panel1Id(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_Panel2Orientation(This, value) \
    ((This)->lpVtbl->get_Panel2Orientation(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_get_Panel2Id(This, value) \
    ((This)->lpVtbl->get_Panel2Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* get_Reading)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs* This,
        __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReading** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_get_Reading(This, value) \
    ((This)->lpVtbl->get_Reading(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_Preview_ITwoPanelHingedDevicePosturePreviewStatics[] = L"Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics* This,
        __FIAsyncOperation_1_Windows__CSystem__CPreview__CTwoPanelHingedDevicePosturePreview** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
    DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#define __x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_GetDefaultAsync(This, result) \
    ((This)->lpVtbl->GetDefaultAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CPreview_CITwoPanelHingedDevicePosturePreviewStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreview
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreview ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreview_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreview_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreview is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreview[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReading ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReading is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReading[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.System.Preview.ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
DEPRECATED("TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_Preview_TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs[] = L"Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
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
#endif // __windows2Esystem2Epreview_p_h__

#endif // __windows2Esystem2Epreview_h__
