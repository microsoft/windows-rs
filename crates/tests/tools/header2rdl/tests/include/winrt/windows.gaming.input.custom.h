
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
#ifndef __windows2Egaming2Einput2Ecustom_h__
#define __windows2Egaming2Einput2Ecustom_h__
#ifndef __windows2Egaming2Einput2Ecustom_p_h__
#define __windows2Egaming2Einput2Ecustom_p_h__


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

#if !defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)
#define WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_GAMING_INPUT_GAMINGINPUTPREVIEWCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Gaming.Input.h"
#include "Windows.Storage.Streams.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface ICustomGameControllerFactory;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory ABI::Windows::Gaming::Input::Custom::ICustomGameControllerFactory

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGameControllerFactoryManagerStatics;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics ABI::Windows::Gaming::Input::Custom::IGameControllerFactoryManagerStatics

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGameControllerFactoryManagerStatics2;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2 ABI::Windows::Gaming::Input::Custom::IGameControllerFactoryManagerStatics2

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGameControllerInputSink;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink ABI::Windows::Gaming::Input::Custom::IGameControllerInputSink

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGameControllerProvider;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider ABI::Windows::Gaming::Input::Custom::IGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGipFirmwareUpdateResult;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult ABI::Windows::Gaming::Input::Custom::IGipFirmwareUpdateResult

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGipGameControllerInputSink;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink ABI::Windows::Gaming::Input::Custom::IGipGameControllerInputSink

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IGipGameControllerProvider;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider ABI::Windows::Gaming::Input::Custom::IGipGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IHidGameControllerInputSink;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink ABI::Windows::Gaming::Input::Custom::IHidGameControllerInputSink

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IHidGameControllerProvider;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider ABI::Windows::Gaming::Input::Custom::IHidGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IXusbGameControllerInputSink;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink ABI::Windows::Gaming::Input::Custom::IXusbGameControllerInputSink

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    interface IXusbGameControllerProvider;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider ABI::Windows::Gaming::Input::Custom::IXusbGameControllerProvider

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    class GipFirmwareUpdateResult;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef struct GipFirmwareUpdateProgress GipFirmwareUpdateProgress;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61b95949-a027-51d8-9f33-37927451502b"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, ABI::Windows::Gaming::Input::Custom::IGipFirmwareUpdateResult*>, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Gaming.Input.Custom.GipFirmwareUpdateResult, Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bfaa48bd-155f-5112-bd86-e01d6f7cd405"))
IAsyncOperationWithProgress<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, ABI::Windows::Gaming::Input::Custom::IGipFirmwareUpdateResult*>, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Gaming.Input.Custom.GipFirmwareUpdateResult, Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("065c16af-49dc-5c94-afe2-9385937facc9"))
IAsyncOperationProgressHandler<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, ABI::Windows::Gaming::Input::Custom::IGipFirmwareUpdateResult*>, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Gaming.Input.Custom.GipFirmwareUpdateResult, Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateResult*, struct ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateProgress> __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                interface IGameController;
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CInput_CIGameController ABI::Windows::Gaming::Input::IGameController

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef enum GipFirmwareUpdateStatus : int GipFirmwareUpdateStatus;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef enum GipMessageClass : int GipMessageClass;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef enum XusbDeviceSubtype : int XusbDeviceSubtype;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef enum XusbDeviceType : int XusbDeviceType;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    typedef struct GameControllerVersionInfo GameControllerVersionInfo;
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    enum GipFirmwareUpdateStatus : int
                    {
                        GipFirmwareUpdateStatus_Completed = 0,
                        GipFirmwareUpdateStatus_UpToDate = 1,
                        GipFirmwareUpdateStatus_Failed = 2,
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipMessageClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    enum GipMessageClass : int
                    {
                        GipMessageClass_Command = 0,
                        GipMessageClass_LowLatency = 1,
                        GipMessageClass_StandardLatency = 2,
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.XusbDeviceSubtype
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    enum XusbDeviceSubtype : int
                    {
                        XusbDeviceSubtype_Unknown = 0,
                        XusbDeviceSubtype_Gamepad = 1,
                        XusbDeviceSubtype_ArcadePad = 2,
                        XusbDeviceSubtype_ArcadeStick = 3,
                        XusbDeviceSubtype_FlightStick = 4,
                        XusbDeviceSubtype_Wheel = 5,
                        XusbDeviceSubtype_Guitar = 6,
                        XusbDeviceSubtype_GuitarAlternate = 7,
                        XusbDeviceSubtype_GuitarBass = 8,
                        XusbDeviceSubtype_DrumKit = 9,
                        XusbDeviceSubtype_DancePad = 10,
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.XusbDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    enum XusbDeviceType : int
                    {
                        XusbDeviceType_Unknown = 0,
                        XusbDeviceType_Gamepad = 1,
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GameControllerVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    struct GameControllerVersionInfo
                    {
                        UINT16 Major;
                        UINT16 Minor;
                        UINT16 Build;
                        UINT16 Revision;
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    struct GipFirmwareUpdateProgress
                    {
                        DOUBLE PercentCompleted;
                        UINT32 CurrentComponentId;
                    };
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.ICustomGameControllerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_ICustomGameControllerFactory[] = L"Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("69a0ae5e-758e-4cbe-ace6-62155fe9126f")
                    ICustomGameControllerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateGameController(
                            ABI::Windows::Gaming::Input::Custom::IGameControllerProvider* provider,
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OnGameControllerAdded(
                            ABI::Windows::Gaming::Input::IGameController* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OnGameControllerRemoved(
                            ABI::Windows::Gaming::Input::IGameController* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICustomGameControllerFactory = __uuidof(ICustomGameControllerFactory);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerFactoryManagerStatics[] = L"Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("36cb66e3-d0a1-4986-a24c-40b137deba9e")
                    IGameControllerFactoryManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE RegisterCustomFactoryForGipInterface(
                            ABI::Windows::Gaming::Input::Custom::ICustomGameControllerFactory* factory,
                            GUID interfaceId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterCustomFactoryForHardwareId(
                            ABI::Windows::Gaming::Input::Custom::ICustomGameControllerFactory* factory,
                            UINT16 hardwareVendorId,
                            UINT16 hardwareProductId
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RegisterCustomFactoryForXusbType(
                            ABI::Windows::Gaming::Input::Custom::ICustomGameControllerFactory* factory,
                            ABI::Windows::Gaming::Input::Custom::XusbDeviceType xusbType,
                            ABI::Windows::Gaming::Input::Custom::XusbDeviceSubtype xusbSubtype
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameControllerFactoryManagerStatics = __uuidof(IGameControllerFactoryManagerStatics);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerFactoryManagerStatics2[] = L"Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("eace5644-19df-4115-b32a-2793e2aea3bb")
                    IGameControllerFactoryManagerStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetFactoryControllerFromGameController(
                            ABI::Windows::Gaming::Input::Custom::ICustomGameControllerFactory* factory,
                            ABI::Windows::Gaming::Input::IGameController* gameController,
                            ABI::Windows::Gaming::Input::IGameController** factoryController
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameControllerFactoryManagerStatics2 = __uuidof(IGameControllerFactoryManagerStatics2);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IGameControllerInputSink";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("1ff6f922-c640-4c78-a820-9a715c558bcb")
                    IGameControllerInputSink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OnInputResumed(
                            UINT64 timestamp
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OnInputSuspended(
                            UINT64 timestamp
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameControllerInputSink = __uuidof(IGameControllerInputSink);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IGameControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("e6d73982-2996-4559-b16c-3e57d46e58d6")
                    IGameControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FirmwareVersionInfo(
                            ABI::Windows::Gaming::Input::Custom::GameControllerVersionInfo* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareProductId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareVendorId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareVersionInfo(
                            ABI::Windows::Gaming::Input::Custom::GameControllerVersionInfo* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsConnected(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameControllerProvider = __uuidof(IGameControllerProvider);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GipFirmwareUpdateResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipFirmwareUpdateResult[] = L"Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("6b794d32-8553-4292-8e03-e16651a2f8bc")
                    IGipFirmwareUpdateResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedErrorCode(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FinalComponentId(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::Input::Custom::GipFirmwareUpdateStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGipFirmwareUpdateResult = __uuidof(IGipFirmwareUpdateResult);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("a2108abf-09f1-43bc-a140-80f899ec36fb")
                    IGipGameControllerInputSink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OnKeyReceived(
                            UINT64 timestamp,
                            BYTE keyCode,
                            boolean isPressed
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE OnMessageReceived(
                            UINT64 timestamp,
                            ABI::Windows::Gaming::Input::Custom::GipMessageClass messageClass,
                            BYTE messageId,
                            BYTE sequenceId,
                            UINT32 messageBufferLength,
                            BYTE* messageBuffer
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGipGameControllerInputSink = __uuidof(IGipGameControllerInputSink);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GipGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IGipGameControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("dbcf1e19-1af5-45a8-bf02-a0ee50c823fc")
                    IGipGameControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SendMessage(
                            ABI::Windows::Gaming::Input::Custom::GipMessageClass messageClass,
                            BYTE messageId,
                            UINT32 messageBufferLength,
                            BYTE* messageBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SendReceiveMessage(
                            ABI::Windows::Gaming::Input::Custom::GipMessageClass messageClass,
                            BYTE messageId,
                            UINT32 requestMessageBufferLength,
                            BYTE* requestMessageBuffer,
                            UINT32 responseMessageBufferLength,
                            BYTE* responseMessageBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateFirmwareAsync(
                            ABI::Windows::Storage::Streams::IInputStream* firmwareImage,
                            __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGipGameControllerProvider = __uuidof(IGipGameControllerProvider);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IHidGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IHidGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("f754c322-182d-40e4-a126-fcee4ffa1e31")
                    IHidGameControllerInputSink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OnInputReportReceived(
                            UINT64 timestamp,
                            BYTE reportId,
                            UINT32 reportBufferLength,
                            BYTE* reportBuffer
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHidGameControllerInputSink = __uuidof(IHidGameControllerInputSink);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IHidGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.HidGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IHidGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IHidGameControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("95ce3af4-abf0-4b68-a081-3b7de73ff0e7")
                    IHidGameControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UsageId(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UsagePage(
                            UINT16* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFeatureReport(
                            BYTE reportId,
                            UINT32 reportBufferLength,
                            BYTE* reportBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SendFeatureReport(
                            BYTE reportId,
                            UINT32 reportBufferLength,
                            BYTE* reportBuffer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SendOutputReport(
                            BYTE reportId,
                            UINT32 reportBufferLength,
                            BYTE* reportBuffer
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHidGameControllerProvider = __uuidof(IHidGameControllerProvider);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IXusbGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IXusbGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("b2ac1d95-6ecb-42b3-8aab-025401ca4712")
                    IXusbGameControllerInputSink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OnInputReceived(
                            UINT64 timestamp,
                            BYTE reportId,
                            UINT32 inputBufferLength,
                            BYTE* inputBuffer
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXusbGameControllerInputSink = __uuidof(IXusbGameControllerInputSink);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IXusbGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.XusbGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IXusbGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IXusbGameControllerProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace Input {
                namespace Custom {
                    MIDL_INTERFACE("6e2971eb-0efb-48b4-808b-837643b2f216")
                    IXusbGameControllerProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetVibration(
                            DOUBLE lowFrequencyMotorSpeed,
                            DOUBLE highFrequencyMotorSpeed
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IXusbGameControllerProvider = __uuidof(IXusbGameControllerProvider);
                } /* Custom */
            } /* Input */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GameControllerFactoryManager_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GameControllerFactoryManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GameControllerFactoryManager[] = L"Windows.Gaming.Input.Custom.GameControllerFactoryManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GipFirmwareUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult[] = L"Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GipGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IGipGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GipGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GipGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GipGameControllerProvider[] = L"Windows.Gaming.Input.Custom.GipGameControllerProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.HidGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IHidGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_HidGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_HidGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_HidGameControllerProvider[] = L"Windows.Gaming.Input.Custom.HidGameControllerProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.Custom.XusbGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IXusbGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_XusbGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_XusbGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_XusbGameControllerProvider[] = L"Windows.Gaming.Input.Custom.XusbGameControllerProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2 __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider;

#endif // ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateProgress __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress* asyncInfo,
        struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CInput_CIGameController __x_ABI_CWindows_CGaming_CInput_CIGameController;

#endif // ____x_ABI_CWindows_CGaming_CInput_CIGameController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateStatus __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateStatus;

typedef enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass;

typedef enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceSubtype __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceSubtype;

typedef enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceType __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceType;

typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGameControllerVersionInfo __x_ABI_CWindows_CGaming_CInput_CCustom_CGameControllerVersionInfo;

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateStatus
{
    GipFirmwareUpdateStatus_Completed = 0,
    GipFirmwareUpdateStatus_UpToDate = 1,
    GipFirmwareUpdateStatus_Failed = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipMessageClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass
{
    GipMessageClass_Command = 0,
    GipMessageClass_LowLatency = 1,
    GipMessageClass_StandardLatency = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.XusbDeviceSubtype
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceSubtype
{
    XusbDeviceSubtype_Unknown = 0,
    XusbDeviceSubtype_Gamepad = 1,
    XusbDeviceSubtype_ArcadePad = 2,
    XusbDeviceSubtype_ArcadeStick = 3,
    XusbDeviceSubtype_FlightStick = 4,
    XusbDeviceSubtype_Wheel = 5,
    XusbDeviceSubtype_Guitar = 6,
    XusbDeviceSubtype_GuitarAlternate = 7,
    XusbDeviceSubtype_GuitarBass = 8,
    XusbDeviceSubtype_DrumKit = 9,
    XusbDeviceSubtype_DancePad = 10,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.XusbDeviceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceType
{
    XusbDeviceType_Unknown = 0,
    XusbDeviceType_Gamepad = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GameControllerVersionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGameControllerVersionInfo
{
    UINT16 Major;
    UINT16 Minor;
    UINT16 Build;
    UINT16 Revision;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateProgress
{
    DOUBLE PercentCompleted;
    UINT32 CurrentComponentId;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.ICustomGameControllerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_ICustomGameControllerFactory[] = L"Windows.Gaming.Input.Custom.ICustomGameControllerFactory";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateGameController)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* provider,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* OnGameControllerAdded)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* value);
    HRESULT (STDMETHODCALLTYPE* OnGameControllerRemoved)(__x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* This,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactoryVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_CreateGameController(This, provider, value) \
    ((This)->lpVtbl->CreateGameController(This, provider, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_OnGameControllerAdded(This, value) \
    ((This)->lpVtbl->OnGameControllerAdded(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_OnGameControllerRemoved(This, value) \
    ((This)->lpVtbl->OnGameControllerRemoved(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerFactoryManagerStatics[] = L"Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterCustomFactoryForGipInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* factory,
        GUID interfaceId);
    HRESULT (STDMETHODCALLTYPE* RegisterCustomFactoryForHardwareId)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* factory,
        UINT16 hardwareVendorId,
        UINT16 hardwareProductId);
    HRESULT (STDMETHODCALLTYPE* RegisterCustomFactoryForXusbType)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* factory,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceType xusbType,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CXusbDeviceSubtype xusbSubtype);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_RegisterCustomFactoryForGipInterface(This, factory, interfaceId) \
    ((This)->lpVtbl->RegisterCustomFactoryForGipInterface(This, factory, interfaceId))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_RegisterCustomFactoryForHardwareId(This, factory, hardwareVendorId, hardwareProductId) \
    ((This)->lpVtbl->RegisterCustomFactoryForHardwareId(This, factory, hardwareVendorId, hardwareProductId))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_RegisterCustomFactoryForXusbType(This, factory, xusbType, xusbSubtype) \
    ((This)->lpVtbl->RegisterCustomFactoryForXusbType(This, factory, xusbType, xusbSubtype))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerFactoryManagerStatics2[] = L"Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetFactoryControllerFromGameController)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2* This,
        __x_ABI_CWindows_CGaming_CInput_CCustom_CICustomGameControllerFactory* factory,
        __x_ABI_CWindows_CGaming_CInput_CIGameController* gameController,
        __x_ABI_CWindows_CGaming_CInput_CIGameController** factoryController);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2Vtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_TryGetFactoryControllerFromGameController(This, factory, gameController, factoryController) \
    ((This)->lpVtbl->TryGetFactoryControllerFromGameController(This, factory, gameController, factoryController))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerFactoryManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IGameControllerInputSink";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnInputResumed)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        UINT64 timestamp);
    HRESULT (STDMETHODCALLTYPE* OnInputSuspended)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink* This,
        UINT64 timestamp);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSinkVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_OnInputResumed(This, timestamp) \
    ((This)->lpVtbl->OnInputResumed(This, timestamp))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_OnInputSuspended(This, timestamp) \
    ((This)->lpVtbl->OnInputSuspended(This, timestamp))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IGameControllerProvider";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FirmwareVersionInfo)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGameControllerVersionInfo* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareProductId)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareVendorId)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareVersionInfo)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        struct __x_ABI_CWindows_CGaming_CInput_CCustom_CGameControllerVersionInfo* value);
    HRESULT (STDMETHODCALLTYPE* get_IsConnected)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProviderVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_get_FirmwareVersionInfo(This, value) \
    ((This)->lpVtbl->get_FirmwareVersionInfo(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_get_HardwareProductId(This, value) \
    ((This)->lpVtbl->get_HardwareProductId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_get_HardwareVendorId(This, value) \
    ((This)->lpVtbl->get_HardwareVendorId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_get_HardwareVersionInfo(This, value) \
    ((This)->lpVtbl->get_HardwareVersionInfo(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_get_IsConnected(This, value) \
    ((This)->lpVtbl->get_IsConnected(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GipFirmwareUpdateResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipFirmwareUpdateResult[] = L"Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedErrorCode)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FinalComponentId)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult* This,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipFirmwareUpdateStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResultVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_get_ExtendedErrorCode(This, value) \
    ((This)->lpVtbl->get_ExtendedErrorCode(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_get_FinalComponentId(This, value) \
    ((This)->lpVtbl->get_FinalComponentId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipFirmwareUpdateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IGipGameControllerInputSink";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnKeyReceived)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        UINT64 timestamp,
        BYTE keyCode,
        boolean isPressed);
    HRESULT (STDMETHODCALLTYPE* OnMessageReceived)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink* This,
        UINT64 timestamp,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass messageClass,
        BYTE messageId,
        BYTE sequenceId,
        UINT32 messageBufferLength,
        BYTE* messageBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSinkVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_OnKeyReceived(This, timestamp, keyCode, isPressed) \
    ((This)->lpVtbl->OnKeyReceived(This, timestamp, keyCode, isPressed))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_OnMessageReceived(This, timestamp, messageClass, messageId, sequenceId, messageBufferLength, messageBuffer) \
    ((This)->lpVtbl->OnMessageReceived(This, timestamp, messageClass, messageId, sequenceId, messageBufferLength, messageBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IGipGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.GipGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IGipGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IGipGameControllerProvider";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SendMessage)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass messageClass,
        BYTE messageId,
        UINT32 messageBufferLength,
        BYTE* messageBuffer);
    HRESULT (STDMETHODCALLTYPE* SendReceiveMessage)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        enum __x_ABI_CWindows_CGaming_CInput_CCustom_CGipMessageClass messageClass,
        BYTE messageId,
        UINT32 requestMessageBufferLength,
        BYTE* requestMessageBuffer,
        UINT32 responseMessageBufferLength,
        BYTE* responseMessageBuffer);
    HRESULT (STDMETHODCALLTYPE* UpdateFirmwareAsync)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* firmwareImage,
        __FIAsyncOperationWithProgress_2_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateResult_Windows__CGaming__CInput__CCustom__CGipFirmwareUpdateProgress** result);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProviderVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_SendMessage(This, messageClass, messageId, messageBufferLength, messageBuffer) \
    ((This)->lpVtbl->SendMessage(This, messageClass, messageId, messageBufferLength, messageBuffer))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_SendReceiveMessage(This, messageClass, messageId, requestMessageBufferLength, requestMessageBuffer, responseMessageBufferLength, responseMessageBuffer) \
    ((This)->lpVtbl->SendReceiveMessage(This, messageClass, messageId, requestMessageBufferLength, requestMessageBuffer, responseMessageBufferLength, responseMessageBuffer))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_UpdateFirmwareAsync(This, firmwareImage, result) \
    ((This)->lpVtbl->UpdateFirmwareAsync(This, firmwareImage, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIGipGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IHidGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IHidGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IHidGameControllerInputSink";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnInputReportReceived)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink* This,
        UINT64 timestamp,
        BYTE reportId,
        UINT32 reportBufferLength,
        BYTE* reportBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSinkVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_OnInputReportReceived(This, timestamp, reportId, reportBufferLength, reportBuffer) \
    ((This)->lpVtbl->OnInputReportReceived(This, timestamp, reportId, reportBufferLength, reportBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IHidGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.HidGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IHidGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IHidGameControllerProvider";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UsageId)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* get_UsagePage)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        UINT16* value);
    HRESULT (STDMETHODCALLTYPE* GetFeatureReport)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        BYTE reportId,
        UINT32 reportBufferLength,
        BYTE* reportBuffer);
    HRESULT (STDMETHODCALLTYPE* SendFeatureReport)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        BYTE reportId,
        UINT32 reportBufferLength,
        BYTE* reportBuffer);
    HRESULT (STDMETHODCALLTYPE* SendOutputReport)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider* This,
        BYTE reportId,
        UINT32 reportBufferLength,
        BYTE* reportBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProviderVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_get_UsageId(This, value) \
    ((This)->lpVtbl->get_UsageId(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_get_UsagePage(This, value) \
    ((This)->lpVtbl->get_UsagePage(This, value))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_GetFeatureReport(This, reportId, reportBufferLength, reportBuffer) \
    ((This)->lpVtbl->GetFeatureReport(This, reportId, reportBufferLength, reportBuffer))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_SendFeatureReport(This, reportId, reportBufferLength, reportBuffer) \
    ((This)->lpVtbl->SendFeatureReport(This, reportId, reportBufferLength, reportBuffer))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_SendOutputReport(This, reportId, reportBufferLength, reportBuffer) \
    ((This)->lpVtbl->SendOutputReport(This, reportId, reportBufferLength, reportBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIHidGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IXusbGameControllerInputSink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerInputSink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IXusbGameControllerInputSink[] = L"Windows.Gaming.Input.Custom.IXusbGameControllerInputSink";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnInputReceived)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink* This,
        UINT64 timestamp,
        BYTE reportId,
        UINT32 inputBufferLength,
        BYTE* inputBuffer);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSinkVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_OnInputReceived(This, timestamp, reportId, inputBufferLength, inputBuffer) \
    ((This)->lpVtbl->OnInputReceived(This, timestamp, reportId, inputBufferLength, inputBuffer))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerInputSink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Gaming.Input.Custom.IXusbGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.Input.Custom.XusbGameControllerProvider
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_Input_Custom_IXusbGameControllerProvider[] = L"Windows.Gaming.Input.Custom.IXusbGameControllerProvider";
typedef struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetVibration)(__x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider* This,
        DOUBLE lowFrequencyMotorSpeed,
        DOUBLE highFrequencyMotorSpeed);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProviderVtbl;

interface __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_SetVibration(This, lowFrequencyMotorSpeed, highFrequencyMotorSpeed) \
    ((This)->lpVtbl->SetVibration(This, lowFrequencyMotorSpeed, highFrequencyMotorSpeed))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CInput_CCustom_CIXusbGameControllerProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GameControllerFactoryManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Gaming.Input.Custom.IGameControllerFactoryManagerStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GameControllerFactoryManager_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GameControllerFactoryManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GameControllerFactoryManager[] = L"Windows.Gaming.Input.Custom.GameControllerFactoryManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GipFirmwareUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IGipFirmwareUpdateResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GipFirmwareUpdateResult[] = L"Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.GipGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IGipGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_GipGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_GipGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_GipGameControllerProvider[] = L"Windows.Gaming.Input.Custom.GipGameControllerProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Gaming.Input.Custom.HidGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IHidGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_HidGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_HidGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_HidGameControllerProvider[] = L"Windows.Gaming.Input.Custom.HidGameControllerProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Gaming.Input.Custom.XusbGameControllerProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.Input.Custom.IXusbGameControllerProvider ** Default Interface **
 *    Windows.Gaming.Input.Custom.IGameControllerProvider
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Gaming_Input_Custom_XusbGameControllerProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_Input_Custom_XusbGameControllerProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_Input_Custom_XusbGameControllerProvider[] = L"Windows.Gaming.Input.Custom.XusbGameControllerProvider";
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
#endif // __windows2Egaming2Einput2Ecustom_p_h__

#endif // __windows2Egaming2Einput2Ecustom_h__
