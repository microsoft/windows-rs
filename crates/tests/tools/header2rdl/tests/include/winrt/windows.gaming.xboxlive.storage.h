
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
#ifndef __windows2Egaming2Exboxlive2Estorage_h__
#define __windows2Egaming2Exboxlive2Estorage_h__
#ifndef __windows2Egaming2Exboxlive2Estorage_p_h__
#define __windows2Egaming2Exboxlive2Estorage_p_h__


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

#if !defined(WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION)
#define WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Gaming.XboxLive.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveBlobGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobGetResult

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveBlobInfo;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfo

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveBlobInfoGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfoGetResult

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveBlobInfoQuery;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfoQuery

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveContainer;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainer

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveContainerInfo;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfo

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveContainerInfoGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoGetResult

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveContainerInfoQuery;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoQuery

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveOperationResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult ABI::Windows::Gaming::XboxLive::Storage::IGameSaveOperationResult

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveProvider;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProvider

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveProviderGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProviderGetResult

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    interface IGameSaveProviderStatics;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProviderStatics

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1___z__zint64_USE
#define DEF___FIAsyncOperation_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cc468085-4bef-5584-907c-9223d2679019"))
IAsyncOperation<__int64> : IAsyncOperation_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__int64> __FIAsyncOperation_1___z__zint64_t;
#define __FIAsyncOperation_1___z__zint64 ABI::Windows::Foundation::__FIAsyncOperation_1___z__zint64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___z__zint64_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___z__zint64_USE
#define DEF___FIAsyncOperationCompletedHandler_1___z__zint64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3ef5872-7d4e-59bb-95ed-79fe0f0dbe89"))
IAsyncOperationCompletedHandler<__int64> : IAsyncOperationCompletedHandler_impl<__int64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Int64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__int64> __FIAsyncOperationCompletedHandler_1___z__zint64_t;
#define __FIAsyncOperationCompletedHandler_1___z__zint64 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___z__zint64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___z__zint64_USE */



#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveBlobGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7023b023-7aed-526c-b3bc-be12e35ce1cf"))
IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*> __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9d96282c-b6ab-5cd3-991b-a358c531bcb6"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobGetResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveBlobInfoGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d7b7f3b4-6028-522f-849d-a69495e4dcd0"))
IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfoGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*> __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9331e53a-a414-51e7-bfbc-7784df83dc8e"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfoGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfoGetResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveContainerInfoGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cff8afeb-5a18-5f51-b61b-943887f729ee"))
IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*> __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("05f86a80-be5b-5e7e-b977-8257c5e48acc"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfoGetResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveOperationResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1c27fb97-1e1a-516f-abb2-12c18e18218d"))
IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveOperationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.XboxLive.Storage.GameSaveOperationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*> __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee53e64f-5319-56fd-a28a-2c474fc42e48"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveOperationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.XboxLive.Storage.GameSaveOperationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveOperationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveProviderGetResult;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE
#define DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3dc36085-5fec-541b-96cf-627b2ad80d36"))
IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProviderGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*> __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_t;
#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7617548d-8e60-50cb-a11e-120fa2082e5b"))
IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProviderGetResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Gaming::XboxLive::Storage::GameSaveProviderGetResult*> __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9114f794-2ceb-5b03-9b22-36884e1f58b3"))
IKeyValuePair<HSTRING, ABI::Windows::Storage::Streams::IBuffer*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Storage::Streams::IBuffer*> __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("790acb62-c4b3-57ea-a152-9e219371c6dc"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3c9ffa92-5123-5ac4-b111-03c215f0c51c"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Storage.Streams.IBuffer>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveBlobInfo;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#define DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aaaf545b-f5e9-5da6-af70-9d904c7a4d37"))
IIterator<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t;
#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#define DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a7c456d7-fa9f-536f-8ed2-459545811ed4"))
IIterable<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t;
#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveContainerInfo;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#define DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0ecd9756-3e0d-523f-a549-2b6504db5202"))
IIterator<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t;
#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#define DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55e4d98f-0889-5c06-a857-7dd168c2d852"))
IIterable<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t;
#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#define DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2cfeec4f-e261-5f4c-aee1-c78518e9d5b9"))
IMapView<HSTRING, ABI::Windows::Storage::Streams::IBuffer*> : IMapView_impl<HSTRING, ABI::Windows::Storage::Streams::IBuffer*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Storage.Streams.IBuffer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Storage::Streams::IBuffer*> __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t;
#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#define DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b9c466a0-2a3f-5f28-a1c1-9cb192f6c786"))
IVectorView<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::XboxLive::Storage::GameSaveBlobInfo*> __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t;
#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#define DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9c490594-0846-50f5-b2ef-c6f03ee6868a"))
IVectorView<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*, ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Gaming::XboxLive::Storage::GameSaveContainerInfo*> __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t;
#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_USE */

#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

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
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    typedef enum GameSaveErrorStatus : int GameSaveErrorStatus;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveBlobInfoQuery;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveContainer;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveContainerInfoQuery;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    class GameSaveProvider;
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    enum GameSaveErrorStatus : int
                    {
                        GameSaveErrorStatus_Ok = 0,
                        GameSaveErrorStatus_Abort = -2147467260,
                        GameSaveErrorStatus_InvalidContainerName = -2138898431,
                        GameSaveErrorStatus_NoAccess = -2138898430,
                        GameSaveErrorStatus_OutOfLocalStorage = -2138898429,
                        GameSaveErrorStatus_UserCanceled = -2138898428,
                        GameSaveErrorStatus_UpdateTooBig = -2138898427,
                        GameSaveErrorStatus_QuotaExceeded = -2138898426,
                        GameSaveErrorStatus_ProvidedBufferTooSmall = -2138898425,
                        GameSaveErrorStatus_BlobNotFound = -2138898424,
                        GameSaveErrorStatus_NoXboxLiveInfo = -2138898423,
                        GameSaveErrorStatus_ContainerNotInSync = -2138898422,
                        GameSaveErrorStatus_ContainerSyncFailed = -2138898421,
                        GameSaveErrorStatus_UserHasNoXboxLiveInfo = -2138898420,
                        GameSaveErrorStatus_ObjectExpired = -2138898419,
                    };
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("917281e0-7201-4953-aa2c-4008f03aef45")
                    IGameSaveBlobGetResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::XboxLive::Storage::GameSaveErrorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveBlobGetResult = __uuidof(IGameSaveBlobGetResult);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfo[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("add38034-baf0-4645-b6d0-46edaffb3c2b")
                    IGameSaveBlobInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Size(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveBlobInfo = __uuidof(IGameSaveBlobInfo);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("c7578582-3697-42bf-989c-665d923b5231")
                    IGameSaveBlobInfoGetResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::XboxLive::Storage::GameSaveErrorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveBlobInfoGetResult = __uuidof(IGameSaveBlobInfoGetResult);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("9fdd74b2-eeee-447b-a9d2-7f96c0f83208")
                    IGameSaveBlobInfoQuery : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetBlobInfoAsync(
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetBlobInfoWithIndexAndMaxAsync(
                            UINT32 startIndex,
                            UINT32 maxNumberOfItems,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetItemCountAsync(
                            __FIAsyncOperation_1_UINT32** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveBlobInfoQuery = __uuidof(IGameSaveBlobInfoQuery);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainer
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainer
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainer[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainer";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("c3c08f89-563f-4ecd-9c6f-33fd0e323d10")
                    IGameSaveContainer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Provider(
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProvider** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SubmitUpdatesAsync(
                            __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* blobsToWrite,
                            __FIIterable_1_HSTRING* blobsToDelete,
                            HSTRING displayName,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReadAsync(
                            __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* blobsToRead,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** action
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAsync(
                            __FIIterable_1_HSTRING* blobsToRead,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SubmitPropertySetUpdatesAsync(
                            ABI::Windows::Foundation::Collections::IPropertySet* blobsToWrite,
                            __FIIterable_1_HSTRING* blobsToDelete,
                            HSTRING displayName,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateBlobInfoQuery(
                            HSTRING blobNamePrefix,
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveBlobInfoQuery** query
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveContainer = __uuidof(IGameSaveContainer);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfo[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("b7e27300-155d-4bb4-b2ba-930306f391b5")
                    IGameSaveContainerInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TotalSize(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LastModifiedTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NeedsSync(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveContainerInfo = __uuidof(IGameSaveContainerInfo);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("ffc50d74-c581-4f9d-9e39-30a10c1e4c50")
                    IGameSaveContainerInfoGetResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::XboxLive::Storage::GameSaveErrorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveContainerInfoGetResult = __uuidof(IGameSaveContainerInfoGetResult);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("3c94e863-6f80-4327-9327-ffc11afd42b3")
                    IGameSaveContainerInfoQuery : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetContainerInfoAsync(
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetContainerInfoWithIndexAndMaxAsync(
                            UINT32 startIndex,
                            UINT32 maxNumberOfItems,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetItemCountAsync(
                            __FIAsyncOperation_1_UINT32** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveContainerInfoQuery = __uuidof(IGameSaveContainerInfoQuery);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveOperationResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveOperationResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("cf0f1a05-24a0-4582-9a55-b1bbbb9388d8")
                    IGameSaveOperationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::XboxLive::Storage::GameSaveErrorStatus* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveOperationResult = __uuidof(IGameSaveOperationResult);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProvider
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProvider[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProvider";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("90a60394-80fe-4211-97f8-a5de14dd95d2")
                    IGameSaveProvider : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_User(
                            ABI::Windows::System::IUser** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateContainer(
                            HSTRING name,
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainer** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeleteContainerAsync(
                            HSTRING name,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** action
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateContainerInfoQuery(
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoQuery** query
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateContainerInfoQueryWithName(
                            HSTRING containerNamePrefix,
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveContainerInfoQuery** query
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetRemainingBytesInQuotaAsync(
                            __FIAsyncOperation_1___z__zint64** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContainersChangedSinceLastSync(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveProvider = __uuidof(IGameSaveProvider);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProviderGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("3ab90816-d393-4d65-ac16-41c3e67ab945")
                    IGameSaveProviderGetResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Gaming::XboxLive::Storage::GameSaveErrorStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            ABI::Windows::Gaming::XboxLive::Storage::IGameSaveProvider** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveProviderGetResult = __uuidof(IGameSaveProviderGetResult);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProviderStatics[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics";
namespace ABI {
    namespace Windows {
        namespace Gaming {
            namespace XboxLive {
                namespace Storage {
                    MIDL_INTERFACE("d01d3ed0-7b03-449d-8cbd-3402842a1048")
                    IGameSaveProviderStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForUserAsync(
                            ABI::Windows::System::IUser* user,
                            HSTRING serviceConfigId,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSyncOnDemandForUserAsync(
                            ABI::Windows::System::IUser* user,
                            HSTRING serviceConfigId,
                            __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGameSaveProviderStatics = __uuidof(IGameSaveProviderStatics);
                } /* Storage */
            } /* XboxLive */
        } /* Gaming */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainer
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainer_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainer[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainer";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveOperationResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics interface starting with version 1.0 of the Windows.Gaming.XboxLive.StorageApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveProvider[] = L"Windows.Gaming.XboxLive.Storage.GameSaveProvider";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics;

#endif // ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1___z__zint64 __FIAsyncOperationCompletedHandler_1___z__zint64;

#if !defined(____FIAsyncOperation_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___z__zint64 __FIAsyncOperation_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___z__zint64;

typedef struct __FIAsyncOperation_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___z__zint64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___z__zint64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___z__zint64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___z__zint64* This,
        __FIAsyncOperationCompletedHandler_1___z__zint64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___z__zint64* This,
        __FIAsyncOperationCompletedHandler_1___z__zint64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___z__zint64* This,
        INT64* result);

    END_INTERFACE
} __FIAsyncOperation_1___z__zint64Vtbl;

interface __FIAsyncOperation_1___z__zint64
{
    CONST_VTBL struct __FIAsyncOperation_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___z__zint64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___z__zint64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___z__zint64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___z__zint64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___z__zint64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___z__zint64_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___z__zint64_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___z__zint64_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___z__zint64 __FIAsyncOperationCompletedHandler_1___z__zint64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___z__zint64;

typedef struct __FIAsyncOperationCompletedHandler_1___z__zint64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___z__zint64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___z__zint64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___z__zint64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___z__zint64* This,
        __FIAsyncOperation_1___z__zint64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___z__zint64Vtbl;

interface __FIAsyncOperationCompletedHandler_1___z__zint64
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___z__zint64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___z__zint64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___z__zint64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___z__zint64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___z__zint64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___z__zint64_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult;

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult;

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult;

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult;

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult;

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult;

typedef struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl;

interface __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

typedef struct __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl;

interface __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

typedef struct __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl;

interface __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

typedef struct __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl;

interface __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

typedef struct __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        __FIIterator_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl;

interface __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer;

typedef struct __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING key,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** first,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl;

interface __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo;

typedef struct __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl;

interface __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo;

typedef struct __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl;

interface __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo_INTERFACE_DEFINED__
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus;

/*
 *
 * Struct Windows.Gaming.XboxLive.Storage.GameSaveErrorStatus
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus
{
    GameSaveErrorStatus_Ok = 0,
    GameSaveErrorStatus_Abort = -2147467260,
    GameSaveErrorStatus_InvalidContainerName = -2138898431,
    GameSaveErrorStatus_NoAccess = -2138898430,
    GameSaveErrorStatus_OutOfLocalStorage = -2138898429,
    GameSaveErrorStatus_UserCanceled = -2138898428,
    GameSaveErrorStatus_UpdateTooBig = -2138898427,
    GameSaveErrorStatus_QuotaExceeded = -2138898426,
    GameSaveErrorStatus_ProvidedBufferTooSmall = -2138898425,
    GameSaveErrorStatus_BlobNotFound = -2138898424,
    GameSaveErrorStatus_NoXboxLiveInfo = -2138898423,
    GameSaveErrorStatus_ContainerNotInSync = -2138898422,
    GameSaveErrorStatus_ContainerSyncFailed = -2138898421,
    GameSaveErrorStatus_UserHasNoXboxLiveInfo = -2138898420,
    GameSaveErrorStatus_ObjectExpired = -2138898419,
};
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResultVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfo[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult* This,
        __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResultVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveBlobInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQueryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetBlobInfoAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetBlobInfoWithIndexAndMaxAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        UINT32 startIndex,
        UINT32 maxNumberOfItems,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobInfoGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetItemCountAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery* This,
        __FIAsyncOperation_1_UINT32** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQueryVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQueryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetBlobInfoAsync(This, operation) \
    ((This)->lpVtbl->GetBlobInfoAsync(This, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetBlobInfoWithIndexAndMaxAsync(This, startIndex, maxNumberOfItems, operation) \
    ((This)->lpVtbl->GetBlobInfoWithIndexAndMaxAsync(This, startIndex, maxNumberOfItems, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_GetItemCountAsync(This, operation) \
    ((This)->lpVtbl->GetItemCountAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainer
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainer
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainer[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainer";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Provider)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider** value);
    HRESULT (STDMETHODCALLTYPE* SubmitUpdatesAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* blobsToWrite,
        __FIIterable_1_HSTRING* blobsToDelete,
        HSTRING displayName,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** operation);
    HRESULT (STDMETHODCALLTYPE* ReadAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        __FIMapView_2_HSTRING_Windows__CStorage__CStreams__CIBuffer* blobsToRead,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** action);
    HRESULT (STDMETHODCALLTYPE* GetAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        __FIIterable_1_HSTRING* blobsToRead,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveBlobGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* SubmitPropertySetUpdatesAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* blobsToWrite,
        __FIIterable_1_HSTRING* blobsToDelete,
        HSTRING displayName,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateBlobInfoQuery)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer* This,
        HSTRING blobNamePrefix,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveBlobInfoQuery** query);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_get_Provider(This, value) \
    ((This)->lpVtbl->get_Provider(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_SubmitUpdatesAsync(This, blobsToWrite, blobsToDelete, displayName, operation) \
    ((This)->lpVtbl->SubmitUpdatesAsync(This, blobsToWrite, blobsToDelete, displayName, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_ReadAsync(This, blobsToRead, action) \
    ((This)->lpVtbl->ReadAsync(This, blobsToRead, action))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_GetAsync(This, blobsToRead, operation) \
    ((This)->lpVtbl->GetAsync(This, blobsToRead, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_SubmitPropertySetUpdatesAsync(This, blobsToWrite, blobsToDelete, displayName, operation) \
    ((This)->lpVtbl->SubmitPropertySetUpdatesAsync(This, blobsToWrite, blobsToDelete, displayName, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_CreateBlobInfoQuery(This, blobNamePrefix, query) \
    ((This)->lpVtbl->CreateBlobInfoQuery(This, blobNamePrefix, query))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfo[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalSize)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LastModifiedTime)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_NeedsSync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_get_TotalSize(This, value) \
    ((This)->lpVtbl->get_TotalSize(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_get_LastModifiedTime(This, value) \
    ((This)->lpVtbl->get_LastModifiedTime(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_get_NeedsSync(This, value) \
    ((This)->lpVtbl->get_NeedsSync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult* This,
        __FIVectorView_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResultVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveContainerInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQueryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetContainerInfoAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetContainerInfoWithIndexAndMaxAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        UINT32 startIndex,
        UINT32 maxNumberOfItems,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveContainerInfoGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetItemCountAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery* This,
        __FIAsyncOperation_1_UINT32** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQueryVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQueryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetContainerInfoAsync(This, operation) \
    ((This)->lpVtbl->GetContainerInfoAsync(This, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetContainerInfoWithIndexAndMaxAsync(This, startIndex, maxNumberOfItems, operation) \
    ((This)->lpVtbl->GetContainerInfoWithIndexAndMaxAsync(This, startIndex, maxNumberOfItems, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_GetItemCountAsync(This, operation) \
    ((This)->lpVtbl->GetItemCountAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveOperationResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveOperationResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult* This,
        enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResultVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveOperationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProvider
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProvider[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProvider";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* CreateContainer)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        HSTRING name,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainer** result);
    HRESULT (STDMETHODCALLTYPE* DeleteContainerAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveOperationResult** action);
    HRESULT (STDMETHODCALLTYPE* CreateContainerInfoQuery)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery** query);
    HRESULT (STDMETHODCALLTYPE* CreateContainerInfoQueryWithName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        HSTRING containerNamePrefix,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveContainerInfoQuery** query);
    HRESULT (STDMETHODCALLTYPE* GetRemainingBytesInQuotaAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        __FIAsyncOperation_1___z__zint64** operation);
    HRESULT (STDMETHODCALLTYPE* get_ContainersChangedSinceLastSync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider* This,
        __FIVectorView_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_CreateContainer(This, name, result) \
    ((This)->lpVtbl->CreateContainer(This, name, result))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_DeleteContainerAsync(This, name, action) \
    ((This)->lpVtbl->DeleteContainerAsync(This, name, action))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_CreateContainerInfoQuery(This, query) \
    ((This)->lpVtbl->CreateContainerInfoQuery(This, query))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_CreateContainerInfoQueryWithName(This, containerNamePrefix, query) \
    ((This)->lpVtbl->CreateContainerInfoQueryWithName(This, containerNamePrefix, query))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_GetRemainingBytesInQuotaAsync(This, operation) \
    ((This)->lpVtbl->GetRemainingBytesInQuotaAsync(This, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_get_ContainersChangedSinceLastSync(This, value) \
    ((This)->lpVtbl->get_ContainersChangedSinceLastSync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProviderGetResult[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        enum __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CGameSaveErrorStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult* This,
        __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResultVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderGetResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Gaming_XboxLive_Storage_IGameSaveProviderStatics[] = L"Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics";
typedef struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUserAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING serviceConfigId,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetSyncOnDemandForUserAsync)(__x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING serviceConfigId,
        __FIAsyncOperation_1_Windows__CGaming__CXboxLive__CStorage__CGameSaveProviderGetResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStaticsVtbl;

interface __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_GetForUserAsync(This, user, serviceConfigId, operation) \
    ((This)->lpVtbl->GetForUserAsync(This, user, serviceConfigId, operation))

#define __x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_GetSyncOnDemandForUserAsync(This, user, serviceConfigId, operation) \
    ((This)->lpVtbl->GetSyncOnDemandForUserAsync(This, user, serviceConfigId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics;
#endif /* !defined(____x_ABI_CWindows_CGaming_CXboxLive_CStorage_CIGameSaveProviderStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfo[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfo";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveBlobInfoQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveBlobInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.GameSaveBlobInfoQuery";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainer
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainer_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainer[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainer";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfo[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfo";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveContainerInfoQuery ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveContainerInfoQuery[] = L"Windows.Gaming.XboxLive.Storage.GameSaveContainerInfoQuery";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveOperationResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveOperationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveOperationResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveOperationResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveProvider
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Gaming.XboxLive.Storage.IGameSaveProviderStatics interface starting with version 1.0 of the Windows.Gaming.XboxLive.StorageApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveProvider ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProvider_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveProvider[] = L"Windows.Gaming.XboxLive.Storage.GameSaveProvider";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult
 *
 * Introduced to Windows.Gaming.XboxLive.StorageApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Gaming.XboxLive.Storage.IGameSaveProviderGetResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult_DEFINED
#define RUNTIMECLASS_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Gaming_XboxLive_Storage_GameSaveProviderGetResult[] = L"Windows.Gaming.XboxLive.Storage.GameSaveProviderGetResult";
#endif
#endif // WINDOWS_GAMING_XBOXLIVE_STORAGEAPICONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Egaming2Exboxlive2Estorage_p_h__

#endif // __windows2Egaming2Exboxlive2Estorage_h__
