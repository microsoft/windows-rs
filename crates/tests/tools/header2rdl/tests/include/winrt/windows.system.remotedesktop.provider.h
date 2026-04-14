
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
#ifndef __windows2Esystem2Eremotedesktop2Eprovider_h__
#define __windows2Esystem2Eremotedesktop2Eprovider_h__
#ifndef __windows2Esystem2Eremotedesktop2Eprovider_p_h__
#define __windows2Esystem2Eremotedesktop2Eprovider_p_h__


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
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IPerformLocalActionRequestedEventArgs;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs ABI::Windows::System::RemoteDesktop::Provider::IPerformLocalActionRequestedEventArgs

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopConnectionInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionInfo

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopConnectionInfo2;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2 ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionInfo2

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopConnectionInfoStatics;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopConnectionRemoteInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionRemoteInfo

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopConnectionRemoteInfoStatics;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionRemoteInfoStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopInfoFactory;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfoFactory

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    interface IRemoteDesktopRegistrarStatics;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopRegistrarStatics

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    class RemoteDesktopInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#define DEF___FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a7175897-18db-5546-bf4f-b5fa4ce8ed85"))
IIterator<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t;
#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#define DEF___FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c98fa56-a539-5a93-b3b2-c7fefacc2e38"))
IIterable<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t;
#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#define DEF___FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("06ef3e49-753e-57de-a409-c3852f347c90"))
IVectorView<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t;
#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#define DEF___FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d009032-ec6b-5789-9a69-eb769e0a1644"))
IVector<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopInfo*> __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t;
#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    class RemoteDesktopConnectionRemoteInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ff5f83b6-18e5-5787-bf03-974ca9929275"))
ITypedEventHandler<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionRemoteInfo*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, IInspectable*> __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    class PerformLocalActionRequestedEventArgs;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6bf735e4-8eaf-53ac-9053-4b2a415428a3"))
ITypedEventHandler<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, ABI::Windows::System::RemoteDesktop::Provider::PerformLocalActionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionRemoteInfo*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::System::RemoteDesktop::Provider::PerformLocalActionRequestedEventArgs*, ABI::Windows::System::RemoteDesktop::Provider::IPerformLocalActionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo, Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionRemoteInfo*, ABI::Windows::System::RemoteDesktop::Provider::PerformLocalActionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

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
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    typedef enum RemoteDesktopConnectionStatus : int RemoteDesktopConnectionStatus;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    typedef enum RemoteDesktopLocalAction : int RemoteDesktopLocalAction;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    class RemoteDesktopConnectionInfo;
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    enum RemoteDesktopConnectionStatus : int
                    {
                        RemoteDesktopConnectionStatus_Connecting = 0,
                        RemoteDesktopConnectionStatus_Connected = 1,
                        RemoteDesktopConnectionStatus_UserInputNeeded = 2,
                        RemoteDesktopConnectionStatus_Disconnected = 3,
                    };
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.System.RemoteDesktop.Provider.RemoteDesktopLocalAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    enum RemoteDesktopLocalAction : int
                    {
                        RemoteDesktopLocalAction_ShowBluetoothSettings = 0,
                        RemoteDesktopLocalAction_ShowSystemSoundSettings = 1,
                        RemoteDesktopLocalAction_ShowSystemDisplaySettings = 2,
                        RemoteDesktopLocalAction_ShowSystemAccountSettings = 3,
                        RemoteDesktopLocalAction_ShowLocalSettings = 4,
                    };
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IPerformLocalActionRequestedEventArgs[] = L"Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("59359f4f-0862-53a3-a3b3-c932fb718cdc")
                    IPerformLocalActionRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Action(
                            ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopLocalAction* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPerformLocalActionRequestedEventArgs = __uuidof(IPerformLocalActionRequestedEventArgs);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("68bd69d6-6dea-543b-b737-f347919f5093")
                    IRemoteDesktopConnectionInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetConnectionStatus(
                            ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopConnectionStatus value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SwitchToLocalSession(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopConnectionInfo = __uuidof(IRemoteDesktopConnectionInfo);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfo2[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("871c0b26-23bf-5d3c-bc35-a85c405e25e6")
                    IRemoteDesktopConnectionInfo2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE PerformLocalActionFromRemote(
                            ABI::Windows::System::RemoteDesktop::Provider::RemoteDesktopLocalAction action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopConnectionInfo2 = __uuidof(IRemoteDesktopConnectionInfo2);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfoStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("4a7dc5a1-3368-5a75-bb78-807df7ebc439")
                    IRemoteDesktopConnectionInfoStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForLaunchUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* launchUri,
                            ABI::Windows::UI::WindowId windowId,
                            ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionInfo** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopConnectionInfoStatics = __uuidof(IRemoteDesktopConnectionInfoStatics);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionRemoteInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("2a3dfa7e-a7ab-547e-9a6a-4c565bbb8d71")
                    IRemoteDesktopConnectionRemoteInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportSwitched(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SwitchToLocalSessionRequested(
                            __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SwitchToLocalSessionRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PerformLocalActionRequested(
                            __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PerformLocalActionRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopConnectionRemoteInfo = __uuidof(IRemoteDesktopConnectionRemoteInfo);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionRemoteInfoStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("b590e64a-e4c9-53e8-b83d-a0db3676246a")
                    IRemoteDesktopConnectionRemoteInfoStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsSwitchSupported(
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetForLaunchUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* launchUri,
                            ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopConnectionRemoteInfo** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopConnectionRemoteInfoStatics = __uuidof(IRemoteDesktopConnectionRemoteInfoStatics);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("d185bb25-2f1e-5098-b9e0-f46d6358c5c4")
                    IRemoteDesktopInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopInfo = __uuidof(IRemoteDesktopInfo);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopInfoFactory[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("ad0e8d58-b56f-5a8b-b419-8002ee0c5ee9")
                    IRemoteDesktopInfoFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING id,
                            HSTRING displayName,
                            ABI::Windows::System::RemoteDesktop::Provider::IRemoteDesktopInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopInfoFactory = __uuidof(IRemoteDesktopInfoFactory);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopRegistrarStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics";
namespace ABI {
    namespace Windows {
        namespace System {
            namespace RemoteDesktop {
                namespace Provider {
                    MIDL_INTERFACE("687c2750-46d9-5de3-8dc3-84a9202cecfb")
                    IRemoteDesktopRegistrarStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DesktopInfos(
                            __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE IsSwitchToLocalSessionEnabled(
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRemoteDesktopRegistrarStatics = __uuidof(IRemoteDesktopRegistrarStatics);
                } /* Provider */
            } /* RemoteDesktop */
        } /* System */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs[] = L"Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo ** Default Interface **
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2 __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics;

#endif // ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

typedef struct __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl;

interface __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

typedef struct __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __FIIterator_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl;

interface __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

typedef struct __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl;

interface __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo;

typedef struct __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __FIVectorView_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** items);

    END_INTERFACE
} __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl;

interface __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo
{
    CONST_VTBL struct __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* This,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* sender,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopConnectionStatus __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopConnectionStatus;

typedef enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopLocalAction __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopLocalAction;

/*
 *
 * Struct Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopConnectionStatus
{
    RemoteDesktopConnectionStatus_Connecting = 0,
    RemoteDesktopConnectionStatus_Connected = 1,
    RemoteDesktopConnectionStatus_UserInputNeeded = 2,
    RemoteDesktopConnectionStatus_Disconnected = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.System.RemoteDesktop.Provider.RemoteDesktopLocalAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopLocalAction
{
    RemoteDesktopLocalAction_ShowBluetoothSettings = 0,
    RemoteDesktopLocalAction_ShowSystemSoundSettings = 1,
    RemoteDesktopLocalAction_ShowSystemDisplaySettings = 2,
    RemoteDesktopLocalAction_ShowSystemAccountSettings = 3,
    RemoteDesktopLocalAction_ShowLocalSettings = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IPerformLocalActionRequestedEventArgs[] = L"Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs* This,
        enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopLocalAction* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIPerformLocalActionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetConnectionStatus)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This,
        enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopConnectionStatus value);
    HRESULT (STDMETHODCALLTYPE* SwitchToLocalSession)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo* This);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_SetConnectionStatus(This, value) \
    ((This)->lpVtbl->SetConnectionStatus(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_SwitchToLocalSession(This) \
    ((This)->lpVtbl->SwitchToLocalSession(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfo2[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* PerformLocalActionFromRemote)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2* This,
        enum __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CRemoteDesktopLocalAction action);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2Vtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_PerformLocalActionFromRemote(This, action) \
    ((This)->lpVtbl->PerformLocalActionFromRemote(This, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionInfoStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForLaunchUri)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* launchUri,
        struct __x_ABI_CWindows_CUI_CWindowId windowId,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_GetForLaunchUri(This, launchUri, windowId, result) \
    ((This)->lpVtbl->GetForLaunchUri(This, launchUri, windowId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionRemoteInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportSwitched)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This);
    HRESULT (STDMETHODCALLTYPE* add_SwitchToLocalSessionRequested)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SwitchToLocalSessionRequested)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PerformLocalActionRequested)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        __FITypedEventHandler_2_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopConnectionRemoteInfo_Windows__CSystem__CRemoteDesktop__CProvider__CPerformLocalActionRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PerformLocalActionRequested)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_ReportSwitched(This) \
    ((This)->lpVtbl->ReportSwitched(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_add_SwitchToLocalSessionRequested(This, handler, token) \
    ((This)->lpVtbl->add_SwitchToLocalSessionRequested(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_remove_SwitchToLocalSessionRequested(This, token) \
    ((This)->lpVtbl->remove_SwitchToLocalSessionRequested(This, token))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_add_PerformLocalActionRequested(This, handler, token) \
    ((This)->lpVtbl->add_PerformLocalActionRequested(This, handler, token))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_remove_PerformLocalActionRequested(This, token) \
    ((This)->lpVtbl->remove_PerformLocalActionRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopConnectionRemoteInfoStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsSwitchSupported)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetForLaunchUri)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* launchUri,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_IsSwitchSupported(This, result) \
    ((This)->lpVtbl->IsSwitchSupported(This, result))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_GetForLaunchUri(This, launchUri, result) \
    ((This)->lpVtbl->GetForLaunchUri(This, launchUri, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopConnectionRemoteInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopInfo[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopInfoFactory[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory* This,
        HSTRING id,
        HSTRING displayName,
        __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactoryVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_CreateInstance(This, id, displayName, value) \
    ((This)->lpVtbl->CreateInstance(This, id, displayName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_System_RemoteDesktop_Provider_IRemoteDesktopRegistrarStatics[] = L"Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics";
typedef struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesktopInfos)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        __FIVector_1_Windows__CSystem__CRemoteDesktop__CProvider__CRemoteDesktopInfo** value);
    HRESULT (STDMETHODCALLTYPE* IsSwitchToLocalSessionEnabled)(__x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStaticsVtbl;

interface __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_get_DesktopInfos(This, value) \
    ((This)->lpVtbl->get_DesktopInfos(This, value))

#define __x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_IsSwitchToLocalSessionEnabled(This, result) \
    ((This)->lpVtbl->IsSwitchToLocalSessionEnabled(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics;
#endif /* !defined(____x_ABI_CWindows_CSystem_CRemoteDesktop_CProvider_CIRemoteDesktopRegistrarStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IPerformLocalActionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_PerformLocalActionRequestedEventArgs[] = L"Windows.System.RemoteDesktop.Provider.PerformLocalActionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfoStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo ** Default Interface **
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfoStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopConnectionRemoteInfo ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopConnectionRemoteInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopConnectionRemoteInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfoFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.System.RemoteDesktop.Provider.IRemoteDesktopInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopInfo[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.System.RemoteDesktop.Provider.IRemoteDesktopRegistrarStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar_DEFINED
#define RUNTIMECLASS_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_System_RemoteDesktop_Provider_RemoteDesktopRegistrar[] = L"Windows.System.RemoteDesktop.Provider.RemoteDesktopRegistrar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esystem2Eremotedesktop2Eprovider_p_h__

#endif // __windows2Esystem2Eremotedesktop2Eprovider_h__
