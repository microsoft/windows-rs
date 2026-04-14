
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
#ifndef __windows2Emedia2Ecapture2Ecore_h__
#define __windows2Emedia2Ecapture2Ecore_h__
#ifndef __windows2Emedia2Ecapture2Ecore_p_h__
#define __windows2Emedia2Ecapture2Ecore_p_h__


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
#include "Windows.Media.Capture.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    interface IVariablePhotoCapturedEventArgs;
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs ABI::Windows::Media::Capture::Core::IVariablePhotoCapturedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    interface IVariablePhotoSequenceCapture;
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture ABI::Windows::Media::Capture::Core::IVariablePhotoSequenceCapture

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    interface IVariablePhotoSequenceCapture2;
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2 ABI::Windows::Media::Capture::Core::IVariablePhotoSequenceCapture2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    class VariablePhotoSequenceCapture;
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2210a640-0e7b-5e8f-a617-2cbb10314a5a"))
ITypedEventHandler<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, ABI::Windows::Media::Capture::Core::IVariablePhotoSequenceCapture*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Capture.Core.VariablePhotoSequenceCapture, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    class VariablePhotoCapturedEventArgs;
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bab30230-3dcd-58e2-aac5-a45f3e6f8097"))
ITypedEventHandler<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, ABI::Windows::Media::Capture::Core::VariablePhotoCapturedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, ABI::Windows::Media::Capture::Core::IVariablePhotoSequenceCapture*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Core::VariablePhotoCapturedEventArgs*, ABI::Windows::Media::Capture::Core::IVariablePhotoCapturedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Capture.Core.VariablePhotoSequenceCapture, Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Capture::Core::VariablePhotoSequenceCapture*, ABI::Windows::Media::Capture::Core::VariablePhotoCapturedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                class CapturedFrame;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                interface ICapturedFrame;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CICapturedFrame ABI::Windows::Media::Capture::ICapturedFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                class CapturedFrameControlValues;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                interface ICapturedFrameControlValues;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues ABI::Windows::Media::Capture::ICapturedFrameControlValues

#endif // ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoCapturedEventArgs[] = L"Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    MIDL_INTERFACE("d1eb4c5c-1b53-4e4a-8b5c-db7887ac949b")
                    IVariablePhotoCapturedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Frame(
                            ABI::Windows::Media::Capture::ICapturedFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CaptureTimeOffset(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UsedFrameControllerIndex(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapturedFrameControlValues(
                            ABI::Windows::Media::Capture::ICapturedFrameControlValues** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVariablePhotoCapturedEventArgs = __uuidof(IVariablePhotoCapturedEventArgs);
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoSequenceCapture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoSequenceCapture[] = L"Windows.Media.Capture.Core.IVariablePhotoSequenceCapture";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    MIDL_INTERFACE("d0112d1d-031e-4041-a6d6-bd742476a8ee")
                    IVariablePhotoSequenceCapture : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE StartAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FinishAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PhotoCaptured(
                            __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PhotoCaptured(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                            __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVariablePhotoSequenceCapture = __uuidof(IVariablePhotoSequenceCapture);
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoSequenceCapture2[] = L"Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Core {
                    MIDL_INTERFACE("fe2c62bc-50b0-43e3-917c-e3b92798942f")
                    IVariablePhotoSequenceCapture2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE UpdateSettingsAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVariablePhotoSequenceCapture2 = __uuidof(IVariablePhotoSequenceCapture2);
                } /* Core */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs[] = L"Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Core.IVariablePhotoSequenceCapture ** Default Interface **
 *    Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoSequenceCapture_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoSequenceCapture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Core_VariablePhotoSequenceCapture[] = L"Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2 __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* This,
        __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* sender,
        __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CICapturedFrame __x_ABI_CWindows_CMedia_CCapture_CICapturedFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues __x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues_FWD_DEFINED__

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoCapturedEventArgs[] = L"Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Frame)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCapture_CICapturedFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_CaptureTimeOffset)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_UsedFrameControllerIndex)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_CapturedFrameControlValues)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCapture_CICapturedFrameControlValues** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_get_Frame(This, value) \
    ((This)->lpVtbl->get_Frame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_get_CaptureTimeOffset(This, value) \
    ((This)->lpVtbl->get_CaptureTimeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_get_UsedFrameControllerIndex(This, value) \
    ((This)->lpVtbl->get_UsedFrameControllerIndex(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_get_CapturedFrameControlValues(This, value) \
    ((This)->lpVtbl->get_CapturedFrameControlValues(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoCapturedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoSequenceCapture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoSequenceCapture[] = L"Windows.Media.Capture.Core.IVariablePhotoSequenceCapture";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCaptureVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* FinishAsync)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_PhotoCaptured)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_Windows__CMedia__CCapture__CCore__CVariablePhotoCapturedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PhotoCaptured)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        __FITypedEventHandler_2_Windows__CMedia__CCapture__CCore__CVariablePhotoSequenceCapture_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCaptureVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCaptureVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_StopAsync(This, operation) \
    ((This)->lpVtbl->StopAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_FinishAsync(This, operation) \
    ((This)->lpVtbl->FinishAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_add_PhotoCaptured(This, handler, token) \
    ((This)->lpVtbl->add_PhotoCaptured(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_remove_PhotoCaptured(This, token) \
    ((This)->lpVtbl->remove_PhotoCaptured(This, token))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Core_IVariablePhotoSequenceCapture2[] = L"Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UpdateSettingsAsync)(__x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_UpdateSettingsAsync(This, operation) \
    ((This)->lpVtbl->UpdateSettingsAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CCore_CIVariablePhotoSequenceCapture2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Core.IVariablePhotoCapturedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Core_VariablePhotoCapturedEventArgs[] = L"Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Capture.Core.VariablePhotoSequenceCapture
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Core.IVariablePhotoSequenceCapture ** Default Interface **
 *    Windows.Media.Capture.Core.IVariablePhotoSequenceCapture2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoSequenceCapture_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Core_VariablePhotoSequenceCapture_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Core_VariablePhotoSequenceCapture[] = L"Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Ecapture2Ecore_p_h__

#endif // __windows2Emedia2Ecapture2Ecore_h__
